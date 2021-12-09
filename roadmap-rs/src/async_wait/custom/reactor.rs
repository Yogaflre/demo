use std::{
    collections::HashMap,
    mem,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    task::Waker,
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
enum Event {
    Close,
    Timeout(usize, u64),
}

pub enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

pub struct Reactor {
    dispatcher: Sender<Event>,
    handler: Option<JoinHandle<()>>,

    pub tasks: HashMap<usize, TaskState>,
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
        self.handler.take().map(|h| h.join().unwrap()).unwrap();
    }
}

impl Reactor {
    pub fn new() -> Arc<Mutex<Self>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Self {
            dispatcher: tx,
            handler: None,
            tasks: HashMap::new(),
        }));

        let reactor_weak = Arc::downgrade(&reactor);
        let handle = thread::spawn(move || {
            let mut handlers = vec![];

            for event in rx {
                println!("receive Event: {:?}", event);
                let reactor_wc = reactor_weak.clone();

                match event {
                    Event::Timeout(id, duration) => {
                        let event_handler = thread::spawn(move || {
                            thread::sleep(Duration::from_millis(duration));
                            if let Some(r) = reactor_wc.upgrade() {
                                r.lock().unwrap().wake(id);
                            }
                        });

                        handlers.push(event_handler);
                    }
                    Event::Close => break,
                };
            }

            // When handle thread be done, we must finish event_handler.
            handlers.into_iter().for_each(|h| h.join().unwrap());
        });
        reactor.lock().unwrap().handler = Some(handle);
        return reactor;
    }

    pub fn register(&mut self, id: usize, duration: u64, waker: Waker) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("task id:{} is exist!", id);
        }
        self.dispatcher.send(Event::Timeout(id, duration)).unwrap();
    }

    pub fn wake(&mut self, id: usize) {
        if let Some(state) = self.tasks.get_mut(&id) {
            match mem::replace(state, TaskState::Ready) {
                TaskState::NotReady(waker) => waker.wake(),
                TaskState::Finished => panic!("can't wake task twice"),
                _ => unimplemented!(),
            }
        }
    }

    pub fn is_ready(&self, id: usize) -> bool {
        return self
            .tasks
            .get(&id)
            .map(|state| match state {
                &TaskState::Ready => true,
                _ => false,
            })
            .unwrap_or(false);
    }
}
