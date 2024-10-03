use std::{thread, sync::{mpsc, Arc, Mutex}};


/// A pool of threads for handling a task.
pub struct ThreadPool<T: Send + 'static> {
    _threads: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<T>,
}
impl<T: Send + 'static> ThreadPool<T> {
    /// Create a new ThreadPool which will handle the given callback.
    ///
    /// Size is the number of threads in the pool.
    pub fn new(
        f: impl FnMut(T) + Send + 'static + Clone,
        size: usize,
    ) -> ThreadPool<T> {
        let mut threads = Vec::with_capacity(size);


        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));


        for _ in 0..size {
            let reciever = Arc::clone(&reciever);
            let mut f = f.clone();
            threads.push(thread::spawn(move || {
                loop {
                    let stream = reciever.lock().unwrap().recv().unwrap();
                    f(stream);
                }
            }));
        }


        ThreadPool { _threads: threads, sender }
    }

    /// Send the given data to the threads.
    pub fn send(&self, data: T) {
        self.sender.send(data).unwrap();
    }
}
