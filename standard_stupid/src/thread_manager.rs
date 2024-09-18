use core::panic;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use tracing::debug;

#[derive(Debug)]
pub struct ThreadPool {
    worker_threads: Vec<ThreadWorker>,
    sender_channel: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new threadpool
    ///
    /// Size of type usize is the amount of threads the pool will hold.
    ///
    /// Will panic the program if size is <= 0.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size is 0 and as such invalid.")
        }
        let (sender_channel, reciever_channel) = mpsc::channel();

        let reciever_channel_safe = Arc::new(Mutex::new(reciever_channel));

        let mut worker_threads = Vec::with_capacity(size);

        for task_id in 0..size {
            worker_threads.push(ThreadWorker::new(
                task_id,
                Arc::clone(&reciever_channel_safe),
            ))
        }

        ThreadPool {
            worker_threads,
            sender_channel: Some(sender_channel),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender_channel.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender_channel.take());
        for worker in &mut self.worker_threads {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct ThreadWorker {
    task_id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadWorker {
    fn new(task_id: usize, reciever_channel: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job_to_do = reciever_channel.lock().unwrap().recv();

            match job_to_do {
                Ok(job) => {
                    debug!("Worker with the {task_id} has recieved a task; executing");

                    job();
                }
                Err(_) => {
                    debug!("Worker {task_id} errored on recieving job, terminating");
                    break;
                }
            }
        });

        ThreadWorker {
            task_id,
            thread: Some(thread),
        }
    }
}
