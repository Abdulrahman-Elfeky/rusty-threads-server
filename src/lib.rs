use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            // any temp structs will be dropped after the let statement and the lock will be
            // released
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
    //fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //    let thread = thread::spawn(move || {
    //        while let Ok(job) = receiver.lock().unwrap().recv() {
    //            println!("Worker {id} got a job; executing.");
    //
    //            job();
    //        }
    //    });
    //
    //    Worker { id, thread }
    //}
}
