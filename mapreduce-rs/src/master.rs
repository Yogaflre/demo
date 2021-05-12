use crate::proto::rpc::{TaskReq, TaskRes, TaskStatus, TaskSyncReq};
use tonic::{transport::Server, Response};

use std::sync::{Arc, Mutex};

use std::collections::HashMap;

use super::proto::rpc::task_service_server::{TaskService, TaskServiceServer};

struct MasterService {
    master: Arc<Mutex<Master>>,
}

impl MasterService {
    fn new(master: Arc<Mutex<Master>>) -> Self {
        return MasterService { master };
    }
}

#[tonic::async_trait]
impl TaskService for MasterService {
    async fn get_task(
        &self,
        request: tonic::Request<TaskReq>,
    ) -> Result<tonic::Response<TaskRes>, tonic::Status> {
        let mut task_res = TaskRes::default();
        if let Ok(mut m) = self.master.lock() {
            match request.into_inner().task_type {
                0 => {
                    task_res.done = m.map_finished;
                    if let Some(v) = m.find_map_tasks() {
                        task_res.files = v;
                    }
                }
                1 => {
                    task_res.done = m.reduce_finished;
                    if let Some((i, v)) = m.find_reduce_tasks() {
                        task_res.hash_code = i;
                        task_res.files = v;
                    }
                }
                t => panic!("Unknown task type. {}", t),
            }
        }
        return Ok(Response::new(task_res));
    }

    async fn sync_task(
        &self,
        request: tonic::Request<TaskSyncReq>,
    ) -> Result<Response<bool>, tonic::Status> {
        let req = request.into_inner();
        if let Ok(mut m) = self.master.lock() {
            match req.task_type {
                0 => {
                    m.map_tasks.iter_mut().for_each(|(k, v)| {
                        if req.input_files.contains(k) {
                            *v = TaskStatus::Finished;
                        }
                    });
                    for (i, path) in req.out_files.into_iter().enumerate() {
                        m.reduce_tasks[i].1.push(path);
                    }
                    m.map_finished = !m.map_tasks.iter().any(|(_, s)| *s != TaskStatus::Finished);
                }
                1 => {
                    m.reduce_tasks[req.hash_code as usize].0 = TaskStatus::Finished;
                    m.result_files.extend(req.out_files);
                    m.reduce_finished = !m
                        .reduce_tasks
                        .iter()
                        .any(|(s, _)| *s != TaskStatus::Finished);
                }
                t => panic!("Unknown task type. {}", t),
            }
        }
        return Ok(Response::new(true));
    }
}

#[derive(Debug)]
struct Master {
    map_tasks: HashMap<String, TaskStatus>,
    map_finished: bool,
    reduce_num: usize,
    reduce_tasks: Vec<(TaskStatus, Vec<String>)>,
    reduce_finished: bool,
    result_files: Vec<String>,
}

impl Master {
    fn init(files: Vec<String>, reduce_num: usize) -> Master {
        let map_tasks = files.into_iter().fold(HashMap::new(), |mut m, f| {
            m.entry(f).or_insert(TaskStatus::Start);
            return m;
        });
        return Master {
            map_tasks,
            map_finished: false,
            reduce_num,
            reduce_tasks: vec![(TaskStatus::Start, vec![]); reduce_num],
            reduce_finished: false,
            result_files: vec![],
        };
    }

    fn find_map_tasks(&mut self) -> Option<Vec<String>> {
        if !self.map_finished {
            return self
                .map_tasks
                .iter_mut()
                .filter(|(_, v)| **v == TaskStatus::Start)
                .nth(0)
                .map(|(k, v)| {
                    *v = TaskStatus::Running;
                    return k.into();
                })
                .map_or(None, |path| Some(vec![path]));
        } else {
            return None;
        }
    }

    fn find_reduce_tasks(&mut self) -> Option<(u32, Vec<String>)> {
        if !self.reduce_finished {
            return self
                .reduce_tasks
                .iter_mut()
                .enumerate()
                .filter(|(_, (s, _))| *s == TaskStatus::Start)
                .nth(0)
                .map(|(i, (s, v))| {
                    *s = TaskStatus::Running;
                    return (i as u32, v.clone());
                })
                .map_or(None, |tuple| Some(tuple));
        } else {
            return None;
        }
    }

    async fn start(
        files: Vec<String>,
        reduce_num: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let master = Arc::new(Mutex::new(Self::init(files, reduce_num)));
        Server::builder()
            .add_service(TaskServiceServer::new(MasterService::new(master.clone())))
            .serve("127.0.0.1:8081".parse()?)
            .await?;
        return Ok(());
    }
}

#[tokio::test]
async fn test() {
    let files = vec!["files/1.txt".to_string(), "files/2.txt".to_string()];
    let reduce_num = 3;
    Master::start(files, reduce_num).await.unwrap();
}
