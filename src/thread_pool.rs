use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "ThreadPool size must be greater than zero");

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { sender, workers }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Message::NewJob(Box::new(job)))
            .expect("ThreadPool workers have stopped");
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = {
                    let receiver = receiver.lock().expect("Worker receiver mutex poisoned");
                    receiver.recv()
                };

                match message {
                    Ok(Message::NewJob(job)) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Ok(Message::Terminate) | Err(_) => {
                        println!("Worker {id} shutting down.");
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination messages to workers...");

        for _ in &self.workers {
            if self.sender.send(Message::Terminate).is_err() {
                eprintln!("Could not send a termination message");
                break;
            }
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take()
                && thread.join().is_err()
            {
                eprintln!("Worker {} panicked", worker.id);
            }
        }

        println!("All workers have shut down.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn executes_jobs() {
        let pool = ThreadPool::new(4);
        let completed = Arc::new(AtomicUsize::new(0));

        for _ in 0..8 {
            let completed = Arc::clone(&completed);

            pool.execute(move || {
                completed.fetch_add(1, Ordering::SeqCst);
            });
        }

        drop(pool);

        assert_eq!(completed.load(Ordering::SeqCst), 8);
    }

    #[test]
    #[should_panic(expected = "ThreadPool size must be greater than zero")]
    fn rejects_zero_workers() {
        ThreadPool::new(0);
    }
}
