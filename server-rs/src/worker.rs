use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use super::message::Message;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    #[cfg(debug_assertions)]
                    println!("Worker {} got a job; executing.", id);

                    job.call_box();

                    #[cfg(debug_assertions)]
                    println!("Worker {} completed its job.", id);
                }
                Message::Terminate => {
                    #[cfg(debug_assertions)]
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
