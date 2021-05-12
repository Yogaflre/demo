use crate::worker::Worker;

#[tokio::test]
async fn woker1() {
    let mapper = Box::new(|text: String| -> Vec<(String, usize)> {
        let mut res: Vec<(String, usize)> = vec![];
        for word in text.split(" ") {
            if !word.is_empty() {
                res.push((word.into(), 1));
            }
        }
        return res;
    });

    let reducer = Box::new(|key: String, values: Vec<usize>| -> (String, usize) {
        return (key, values.into_iter().sum());
    });

    let mut woker = Worker::init("worker1".to_string(), 3, mapper, reducer);
    woker.start().await;
}

#[tokio::test]
async fn woker2() {
    let mapper = Box::new(|text: String| -> Vec<(String, usize)> {
        let mut res: Vec<(String, usize)> = vec![];
        for word in text.split(" ") {
            if !word.is_empty() {
                res.push((word.into(), 1));
            }
        }
        return res;
    });
    let reducer = Box::new(|key: String, values: Vec<usize>| -> (String, usize) {
        return (key, values.into_iter().sum());
    });

    let mut woker = Worker::init("worker2".to_string(), 3, mapper, reducer);
    woker.start().await;
}

#[tokio::test]
async fn woker3() {
    let mapper = Box::new(|text: String| -> Vec<(String, usize)> {
        let mut res: Vec<(String, usize)> = vec![];
        for word in text.split(" ") {
            if !word.is_empty() {
                res.push((word.into(), 1));
            }
        }
        return res;
    });
    let reducer = Box::new(|key: String, values: Vec<usize>| -> (String, usize) {
        return (key, values.into_iter().sum());
    });

    let mut woker = Worker::init("worker3".to_string(), 3, mapper, reducer);
    woker.start().await;
}
