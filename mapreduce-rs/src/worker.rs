use std::{
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, BTreeMap, BinaryHeap},
    fmt::{Debug, Display},
    fs::{self, File},
    hash::Hash,
    hash::Hasher,
    io::Lines,
    io::{prelude::*, BufReader, SeekFrom},
    iter::Peekable,
    mem::size_of_val,
    path::Path,
    str::FromStr,
    thread,
    time::Duration,
};

use crate::proto::rpc::{task_service_client::TaskServiceClient, TaskReq, TaskRes, TaskSyncReq};

pub trait Reducer {
    fn reduce(&self);
}

#[derive(Default)]
pub struct Worker<K, V, Mapper, Reducer>
where
    K: Hash + Display + FromStr,
    V: Display + FromStr,
    <K as FromStr>::Err: Debug,
    <V as FromStr>::Err: Debug,
    Mapper: Fn(String) -> Vec<(K, V)>,
    Reducer: Fn(K, Vec<V>) -> (K, V),
{
    name: String,
    mapper: Box<Mapper>,
    reducer: Box<Reducer>,
    reduce_num: u32,
    map_finished: bool,
    reduce_finished: bool,
}

impl<K, V, Mapper, Reducer> Worker<K, V, Mapper, Reducer>
where
    K: Hash + Display + FromStr,
    V: Display + FromStr,
    <K as FromStr>::Err: Debug,
    <V as FromStr>::Err: Debug,
    Mapper: Fn(String) -> Vec<(K, V)>,
    Reducer: Fn(K, Vec<V>) -> (K, V),
{
    pub fn init(name: String, reduce_num: u32, mapper: Box<Mapper>, reducer: Box<Reducer>) -> Self {
        return Worker {
            name,
            mapper,
            reducer,
            reduce_num,
            map_finished: false,
            reduce_finished: false,
        };
    }

    pub async fn start(&mut self) {
        let mut client = TaskServiceClient::connect("http://localhost:8081")
            .await
            .unwrap();
        while !self.reduce_finished {
            self.handle(&mut client).await;
            thread::sleep(Duration::from_secs(3));
        }
        println!("worker shutdown!");
    }

    async fn handle(&mut self, client: &mut TaskServiceClient<tonic::transport::Channel>) {
        if !self.map_finished {
            let req = TaskReq { task_type: 0 };
            let res = client.get_task(req).await.unwrap().into_inner();
            self.handle_map(client, res).await;
        } else {
            let req = TaskReq { task_type: 1 };
            let res = client.get_task(req).await.unwrap().into_inner();
            self.handle_reduce(client, res).await;
        }
    }

    async fn handle_map(
        &mut self,
        client: &mut TaskServiceClient<tonic::transport::Channel>,
        response: TaskRes,
    ) {
        if response.done {
            self.map_finished = true;
        } else if !response.files.is_empty() {
            let paths: Vec<String> = (0..self.reduce_num)
                .into_iter()
                .map(|n| format!("files/map/{}/{}.txt", self.name, n))
                .collect();
            let mut files: Vec<fs::File> =
                paths.iter().map(|path| Self::get_new_file(path)).collect();
            for path in response.files.iter() {
                for line in BufReader::new(fs::File::open(path).unwrap()).lines() {
                    if let Ok(l) = line {
                        let kvs: Vec<(K, V)> = (self.mapper)(l);
                        kvs.iter().for_each(|(k, v)| {
                            let mut hasher = DefaultHasher::new();
                            k.hash(&mut hasher);
                            let file: &mut fs::File = files
                                .get_mut(hasher.finish() as usize % self.reduce_num as usize)
                                .unwrap();
                            file.write_all(format!("{} {}\n", k, v).as_bytes()).unwrap();
                        });
                    }
                }
            }
            let sync_req = TaskSyncReq {
                task_type: 0,
                hash_code: 0,
                input_files: response.files,
                out_files: paths,
            };
            client.sync_task(sync_req.clone()).await.unwrap();
        }
    }

    async fn handle_reduce(
        &mut self,
        client: &mut TaskServiceClient<tonic::transport::Channel>,
        response: TaskRes,
    ) {
        if response.done {
            self.reduce_finished = true;
        } else if !response.files.is_empty() {
            // 获取到排好序的文件
            let sorted_files: Vec<File> = self
                .shuffle(&response.files)
                .into_iter()
                .map(|path| File::open(path).unwrap())
                .collect();

            let merge_path = format!("files/tmp/{}/{}.txt", self.name, self.name);
            Self::merge(sorted_files, &merge_path);

            let output_path = format!("files/reduce/{}_{}.txt", self.name, response.hash_code);
            let mut output_file = Self::get_new_file(&output_path);
            // FIXME 这里一次读取一整行可能会导致内存溢出，可以修改为读取数据流
            for line in BufReader::new(File::open(merge_path).unwrap()).lines() {
                if let Ok(l) = line {
                    let split: Vec<&str> = l.splitn(2, " ").collect();
                    let key: K = split[0].parse::<K>().unwrap();
                    let values: Vec<V> = split[1]
                        .split(" ")
                        .map(|x| x.parse::<V>().unwrap())
                        .collect();
                    let kv: (K, V) = (self.reducer)(key, values);
                    output_file
                        .write_all(format!("{} {}\n", kv.0, kv.1).as_bytes())
                        .unwrap();
                }
            }

            fs::remove_dir_all(format!("files/tmp/{}", self.name)).unwrap();
            let sync_req = TaskSyncReq {
                task_type: 1,
                hash_code: response.hash_code,
                input_files: response.files,
                out_files: vec![output_path],
            };
            client.sync_task(sync_req).await.unwrap();
        }
    }

    fn shuffle(&self, files: &Vec<String>) -> Vec<String> {
        return files
            .into_iter()
            .enumerate()
            .map(|(index, path)| {
                let mut tmp_files: Vec<fs::File> = vec![];
                let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
                let mut lines = BufReader::new(File::open(path).unwrap()).lines();
                let mut flag = true;
                while flag {
                    flag = false;
                    if let Some(Ok(l)) = lines.next() {
                        let split: Vec<&str> = l.splitn(2, " ").collect();
                        let kv: (String, String) = (split[0].into(), split[1].into());
                        map.entry(kv.0).or_insert(vec![]).push(kv.1);
                        flag = true;
                    }
                    // NOTE 模拟限制一批使用内存16bytes
                    if size_of_val(&map) >= 16 || !flag {
                        let mut tmp = Self::get_new_file(&format!(
                            "files/tmp/{}/{}_{}.txt",
                            self.name,
                            index,
                            tmp_files.len()
                        ));
                        map.iter().for_each(|(k, v)| {
                            let mut s = String::new();
                            s.push_str(k);
                            v.into_iter().for_each(|value| {
                                s.push(' ');
                                s.push_str(value);
                            });
                            s.push_str("\n");
                            tmp.write_all(s.as_bytes()).unwrap();
                        });
                        tmp_files.push(tmp);
                        map.clear();
                    }
                }

                // 归并排序临时文件，并聚合相同的key
                let output_path = format!("files/tmp/{}/{}.txt", self.name, index);
                Self::merge(tmp_files, &output_path);
                return output_path;
            })
            .collect();
    }

    fn merge(mut input_files: Vec<File>, output_path: &String) {
        input_files.iter_mut().for_each(|file| {
            file.seek(SeekFrom::Start(0)).unwrap();
        });
        let mut output_file = Self::get_new_file(output_path);

        let mut peekable_lines: Vec<Peekable<Lines<BufReader<File>>>> = input_files
            .into_iter()
            .map(|f| BufReader::new(f).lines().peekable())
            .collect();
        let mut tmp_heap: BinaryHeap<Reverse<(String, usize)>> = BinaryHeap::new();

        for (i, lines) in peekable_lines.iter_mut().enumerate() {
            if let Some(Ok(line)) = lines.peek() {
                let kv: Vec<&str> = line.splitn(2, " ").collect();
                tmp_heap.push(Reverse((kv[0].into(), i)));
            }
        }

        let mut pre = None;
        while !tmp_heap.is_empty() {
            let pop = tmp_heap.pop();
            if let Some(Reverse((ref pop_word, pop_index))) = pop {
                let mut pop_line = peekable_lines[pop_index].next().unwrap().unwrap();
                if let Some(Reverse((ref pre_word, _))) = pre {
                    if pop_word == pre_word {
                        output_file.write_all(b" ").unwrap();
                        pop_line = pop_line.splitn(2, " ").nth(1).unwrap().to_string();
                    } else {
                        output_file.write_all(b"\n").unwrap();
                    }
                }
                pre = pop;
                output_file.write_all(pop_line.as_bytes()).unwrap();
                if let Some(Ok(line)) = peekable_lines[pop_index].peek() {
                    tmp_heap.push(Reverse((
                        line.splitn(2, " ").nth(0).unwrap().to_string(),
                        pop_index,
                    )));
                }
            }
        }
    }

    fn get_new_file(path: &str) -> fs::File {
        let path = Path::new(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        return fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();
    }
}
