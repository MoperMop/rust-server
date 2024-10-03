use std::{thread, sync::{mpsc, Arc, Mutex}};


/// A pool of threads for handling a task.
pub struct ThreadPool<T: Send + 'static> {
    threads: Vec<Option<thread::JoinHandle<()>>>,
    sender: Option<mpsc::Sender<T>>,
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
            threads.push(Some(thread::spawn(move || {
                loop {
                    match reciever.lock().unwrap().recv() {
                        Ok(stream) => f(stream),
                        Err(_) => break,
                    }
                }
            })));
        }


        ThreadPool { threads, sender: Some(sender) }
    }

    /// Send the given data to the threads.
    pub fn send(&self, data: T) {
        self.sender.as_ref().unwrap().send(data).unwrap();
    }
}

impl<T: Send + 'static> Drop for ThreadPool<T> {
    fn drop(&mut self) {
        drop(self.sender.take());

        for thread in &mut self.threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
