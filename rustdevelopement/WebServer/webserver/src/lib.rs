// Create ThreadPool struct
pub struct ThreadPool;
// impl block
impl ThreadPool {
    // new method
    pub fn new(_size: usize) -> ThreadPool {
        // create a new ThreadPool
        ThreadPool
    }
    // execute method
    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

