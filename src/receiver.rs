// need aws crates here
// need logging crates here, idt theres a built-in

/// The Receiver struct can be used to receive data from a Rust channel.
/// It has two attributes, a rx endpoint to receive data of type T and a 
/// buffer to add the data to.
struct Receiver {
    rx: Receiver<T>,
    buffer: std::Sync::Mutex<Vec<T>>,
    buffer_capacity: i32,
}

/// The Receiver implementation consists of a receive data method which
/// acquires the mutex on the Receiver buffer and adds data to it. If the 
/// mutex is poisoned, we unpoison it before continuing.
impl Receiver {

    /// Receives data from a channel and adds it to a buffer. If the buffer is
    /// over capacity, we upload the data to AWS.
    fn receive_data(&self) {
        loop {
            let data = self.rx.recv(); // gets data from channel

            // deals with getting a mutex
            let mut buffer = self.buffer.lock().unwrap_or_else(|mut e| {
                // log that mutex was poisoned, ask what logging crate we're using
                self.buffer.clear_poison(); // clears mutex poisoning and adds mutex guard
                let mut buffer = self.buffer.lock().unwrap();
                buffer
            });

            buffer.push(data); // pushes data to the buffer

            if buffer.len() > buffer_capacity { // uploads data to aws if over buffer capacity
                self.upload_data();
            }
        }
    }

    /// Uploads data to an AWS service by getting a lock on the buffer Mutex,
    /// sending the data to AWS, then clearing the buffer for future use.
    fn upload_data(&self) {
        // acquire lock on buffer
        let mut buffer = self.buffer.lock().unwrap_or_else(|mut e| {
                // log that mutex was poisoned
                self.buffer.clear_poison();
                let mut buffer = self.buffer.lock().unwrap();
                buffer
            });

        // code that uploads to aws and stuff

        buffer.clear(); // clear buffer for repeated use
    }
}