use std::thread;


/// A pool of threads for handling tasks.
pub struct ThreadPool;
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// Size is the number of threads in the pool.
    pub fn new(_size: usize) -> ThreadPool {
        ThreadPool
    }

    /// Add the given callback to the queue.
    pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
        thread::spawn(f);
    }
}
