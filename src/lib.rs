use std::{thread, net::TcpStream, sync::{mpsc, Arc, Mutex}};


/// A pool of threads for handling tasks.
pub struct ThreadPool {
    _threads: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<TcpStream>,
}
impl ThreadPool {
    /// Create a new ThreadPool which will handle the given callback.
    ///
    /// Size is the number of threads in the pool.
    pub fn new(
        f: impl FnMut(TcpStream) + Send + 'static + Clone,
        size: usize,
    ) -> ThreadPool {
        let mut threads = Vec::with_capacity(size);


        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));


        for _ in 0..size {
            let reciever = Arc::clone(&reciever);
            let mut f = (f).clone();
            threads.push(thread::spawn(move || {
                loop {
                    let stream = reciever.lock().unwrap().recv().unwrap();
                    f(stream);
                }
            }));
        }


        ThreadPool { _threads: threads, sender }
    }

    /// Add the given callback to the queue.
    pub fn execute(&self, stream: TcpStream) {
        self.sender.send(stream).unwrap();
    }
}
