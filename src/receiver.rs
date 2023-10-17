// need aws crates here
// need logging crates here, idt theres a built-in

/// The Receiver struct can be used to receive data from a Rust channel.
/// It has two attributes, a rx endpoint to receive data of type T and a 
/// buffer to add the data to.
struct Receiver {
    rx: Receiver<T>,
    buffer: std::Sync::Mutex<Vec<T>>,
}

/// The Receiver implementation consists of a receive data method which
/// acquires the mutex on the Receiver buffer and adds data to it. If the 
/// mutex is poisoned, we unpoison it before continuing.
impl Receiver {
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
        }
    }

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