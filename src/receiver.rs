// need aws crates here
// need logging crates here, idt theres a built-in

/// The Receiver struct can be used to receive data from a Rust channel.
/// It has two attributes, a rx endpoint to receive data of type T and a 
/// buffer to add the data to.
struct Receiver {
    rx: Receiver<T>,
    buffer: Vec<T>,
    buffer_capacity: i32,
}

/// The Receiver implementation consists of receive_data and upload_data
/// methods. The receive_data method receives data from a Rust channel and pushes
/// it to a buffer. The upload_data method uploads the data to an AWS service then
/// clears it.
impl Receiver {

    /// Receives data from a channel and adds it to a buffer. If the buffer is
    /// over capacity, we upload the data to AWS.
    fn receive_data(&self) {
        loop {
            let data = self.rx.recv(); // gets data from channel

            // deals with getting a mutex
            let mut buffer = self.buffer;

            buffer.push(data); // pushes data to the buffer

            if buffer.len() > buffer_capacity { // uploads data to aws if over buffer capacity
                self.upload_data();
            }
        }
    }

    /// Uploads data to an AWS service by getting the buffer, sending the
    /// data to AWS, then clearing the buffer.
    fn upload_data(&self) {
        let mut buffer = self.buffer;

        // code that uploads to aws and stuff

        buffer.clear(); // clear buffer for repeated use
    }
}