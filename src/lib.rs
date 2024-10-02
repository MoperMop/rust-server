use std::thread;


/// A pool of threads for handling tasks.
pub struct ThreadPool {
    _workers: Vec<Worker>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// Size is the number of threads in the pool.
    pub fn new(size: usize) -> ThreadPool {
        let mut _workers = Vec::with_capacity(size);


        for _ in 0..size {
            _workers.push(Worker::new());
        }


        ThreadPool { _workers }
    }

    /// Add the given callback to the queue.
    pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
        thread::spawn(f);
    }
}

struct Worker{
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new() -> Worker {
        Worker { thread: thread::spawn(|| {}) }
    }
}
