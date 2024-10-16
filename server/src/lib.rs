use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(_id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", _id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} got a shutdown signal; shutting down.", _id);
                    break;
                }
            }
        });
        Worker {
            id: _id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        assert!(_size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(_size);

        for idx in 0.._size {
            let worker = Worker::new(idx, Arc::clone(&receiver));
            workers.push(worker)
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(_f);
        if let Some(ref sender) = self.sender {
            sender.send(job).unwrap();
        }
    }
}
