use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, &str> {
        assert!(size > 0);
        // match size > 0 {
        //     true => Ok(ThreadPool),
        //     false => Err("Thread pool size is 0")
        // }
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id))
        }
        ThreadPool {workers}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread}
    }
}