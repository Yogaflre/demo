use crate::threadpool::ThreadPool;
use crate::timer::TimerType::Repeating;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type F = Arc<dyn Fn() + Send + Sync>;

enum TimerType {
    Delay,
    Repeating(u128),
}

struct Task {
    pub timer_type: TimerType,
    pub timestamp: u128,
    pub event: F,
}

impl Task {
    pub fn new(timer_type: TimerType, timestamp: u128, event: F) -> Self {
        Task {
            timer_type: timer_type,
            timestamp: timestamp,
            event: event,
        }
    }
}

enum Message {
    Task(Task),
    Stop,
}

pub struct Timer {
    thread: Option<thread::JoinHandle<()>>,
    sender: Arc<Mutex<Sender<Message>>>,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.sender
            .lock()
            .expect("locked error")
            .send(Message::Stop)
            .expect("send stop error.");
        if let Some(thread) = self.thread.take() {
            thread.join().expect("join timer thread error.");
        }
    }
}

impl Timer {
    pub fn new(size: u8) -> Self {
        let (sender, receiver) = channel::<Message>();
        let send: Arc<Mutex<Sender<Message>>> = Arc::new(Mutex::new(sender));

        let threadpool = ThreadPool::new(size);
        let s = send.clone();
        let thread = std::thread::spawn(move || {
            Timer::run(threadpool, s, Arc::new(Mutex::new(receiver)));
        });

        let timer = Timer {
            thread: Some(thread),
            sender: send,
        };
        return timer;
    }

    fn run(
        threadpool: ThreadPool,
        sender: Arc<Mutex<Sender<Message>>>,
        receiver: Arc<Mutex<Receiver<Message>>>,
    ) {
        while let Ok(message) = receiver.lock().unwrap().recv() {
            match message {
                Message::Task(mut task) => {
                    let now: u128 = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    if task.timestamp <= now {
                        match task.timer_type {
                            Repeating(m) => {
                                threadpool.execute_arc(task.event.clone());
                                task.timestamp = now + m;
                                sender.lock().unwrap().send(Message::Task(task)).unwrap();
                            }
                            _ => threadpool.execute_arc(task.event),
                        }
                    } else {
                        // TODO 未到时间再插入channel，可以考虑其他的方式
                        sender.lock().unwrap().send(Message::Task(task)).unwrap();
                    }
                }
                Message::Stop => break,
            }
        }
    }

    pub fn delay(&mut self, duration: Duration, event: F) {
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        timestamp += duration.as_millis();
        self.add(TimerType::Delay, timestamp, event);
    }

    pub fn date(&mut self, duration: Duration, event: F) {
        self.add(TimerType::Delay, duration.as_millis(), event);
    }

    pub fn repeating(&mut self, duration: Duration, event: F) {
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        timestamp += duration.as_millis();
        self.add(TimerType::Repeating(duration.as_millis()), timestamp, event);
    }

    fn add(&mut self, timer_type: TimerType, timestamp: u128, event: F) {
        let task = Task::new(timer_type, timestamp, event);
        self.sender
            .lock()
            .unwrap()
            .send(Message::Task(task))
            .unwrap();
    }
}
