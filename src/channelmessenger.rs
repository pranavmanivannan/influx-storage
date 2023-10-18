use crate::data;
use std::sync::mpsc;

/// A struct for setting a channel sender endpoint.
pub struct ChannelMessenger<T>
where
    T: data::Data,
{
    pub sender_endpoint: mpsc::Sender<T>,
}

/// An implementation of ChannelMessenger with a constructor and send_data method.
impl<T> ChannelMessenger<T>
where
    T: data::Data + Clone,
{
    /// Basic constructor for ChannelMessenger that takes in a Sender<T> endpoint as input
    pub fn new(endpoint: mpsc::Sender<T>) -> ChannelMessenger<T> {
        ChannelMessenger {
            sender_endpoint: endpoint,
        }
    }

    /// A custom send method which takes in data and send it to a channel. If unable to send to the channel,
    /// it will print the error that occurs instead of panicking.
    pub fn send_data(&self, data: &T) {
        let data_copy: T = data.clone();
        match self.sender_endpoint.send(data_copy) {
            Ok(_) => {}
            Err(e) => println!("Unable to send information to channel {:?}", e),
        }
        println!("Sender work"); // for testing
    }
}
