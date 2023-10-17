/// The Sender struct can be used to send data to a Rust channel. 
/// It has one attribute, a tx endpoint of type T which is the type of message to 
/// be sent to the channel.
struct Sender {
    tx: Sender<T>
}

/// The Sender implementation consists of a custom send method which takes data and sends it
/// to a channel. In the case the function cannot send data due to an error, it will print an
/// error statement and allow the program to continue executing.
impl Sender {
    fn send_data(&self, data: T) {
        match self.tx.send(data) {
            Ok(_) => {
                
            },
            Err(e) => {
                // log that we were unable to send information
            }
        }
    }
}