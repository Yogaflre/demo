// use std::{
//     any::Any,
//     future::Future,
//     mem,
//     pin::Pin,
//     sync::{
//         mpsc::{self, Receiver, SyncSender},
//         Arc, Mutex,
//     },
//     task::{self, Context, RawWaker, Wake, Waker},
// };

// struct Task {
//     future: Mutex<Option<Pin<Box<dyn Future<Output = dyn Any>>>>>,
//     task_sender: SyncSender<Arc<Task>>,
// }

// impl Wake for Task {
//     fn wake(self: Arc<Self>) {
//         let cloned = self.clone();
//         self.task_sender
//             .send(cloned)
//             .expect("channel size is not enough");
//     }
// }

// struct Spawner {
//     task_sender: SyncSender<Arc<Task>>,
// }

// impl Spawner {
//     fn spawn(&self, f: impl Future<Output = dyn Any> + 'static) {
//         let bf = Box::pin(f);
//         let task = Arc::new(Task {
//             future: Mutex::new(Some(bf)),
//             task_sender: self.task_sender.clone(),
//         });
//         self.task_sender
//             .send(task)
//             .expect("channel size is not enough");
//     }
// }

// struct Executor {
//     ready_queue: Receiver<Arc<Task>>,
// }

// impl Executor {
//     fn run(&self) {
//         while let Ok(task) = self.ready_queue.recv() {
//             let f = task.future.lock().unwrap();
//             if let Some(future) = f.take() {
//                 let raw_waker: RawWaker = mem::transmute(task as &dyn Wake);
//                 let waker = Waker::from_raw(raw_waker);
//                 let context = Context::from_waker(&waker);
//                 if future.as_mut().poll(&mut context).is_pending() {
//                     *f = Some(future);
//                 }
//             }
//         }
//     }
// }

// fn create_executor() -> (Executor, Spawner) {
//     let (tx, rx): (SyncSender<Arc<Task>>, Receiver<Arc<Task>>) = mpsc::sync_channel(5);
//     return (Executor { ready_queue: rx }, Spawner { task_sender: tx });
// }
