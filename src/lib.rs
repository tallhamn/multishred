use std::thread;
use std::sync::mpsc;



pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<job>,
}

struct job;

impl ThreadPool{
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size:usize) -> ThreadPool{
        assert!(size >0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver))

        }

        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {

    }


}


struct  Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{

    fn new (id: usize, receiver: mpsc::Receiver<job>) -> Worker{
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }

    }

}