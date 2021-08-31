pub struct ThreadPool;

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
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}