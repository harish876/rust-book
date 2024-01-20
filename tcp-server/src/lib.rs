use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();

            println!("Worker {} is executing the job", id);

            job();
        });

        Self { id, thread }
    }
}

pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        let (sender, reciever) = mpsc::channel();
        let reciever: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(reciever));

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&reciever));
            workers.push(worker);
        }

        Self {
            size,
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        //now send the job to a worker

        match self.sender.send(job) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }
}

fn climb_stairs(n: usize) -> usize {
    if n == 1 || n == 0 {
        return 1;
    }

    return climb_stairs(n - 1) + climb_stairs(n - 2);
}

fn climb_stairs_memo(n: i32) -> i32 {
    let size = (n + 1).try_into().unwrap();
    let mut memo: Vec<usize> = Vec::with_capacity(size);
    memo.push(1);
    memo.push(1);

    for i in 2..n + 1 {
        let i:usize = i.try_into().unwrap();
        memo.push(memo[i - 1] + memo[i - 2]);
    }

    return memo[size-1].try_into().unwrap();
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn climb_stairs_test_one() {
        assert!(climb_stairs(2) == 2);
    }
    #[test]
    fn climb_stairs_test_two() {
        println!("{}", climb_stairs_memo(44));
        assert!(climb_stairs_memo(44) == 1134903170)
    }
}
