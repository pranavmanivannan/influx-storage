use crate::data;
use std::sync::mpsc;

pub struct ChannelMessenger<T>
where
    T: data::Data,
{
    pub sender_endpoint: mpsc::Sender<T>,
}

impl<T> ChannelMessenger<T>
where
    T: data::Data + Clone,
{
    pub fn new(endpoint: mpsc::Sender<T>) -> ChannelMessenger<T> {
        ChannelMessenger {
            sender_endpoint: endpoint,
        }
    }

    pub fn send_data(&self, data: &T) {
        let data_copy: T = data.clone();
        match self.sender_endpoint.send(data_copy) {
            Ok(_) => {},
            Err(e) => println!("Unable to send information to channel {:?}", e),
        }
        println!("Sender work"); // for testing
    }
}
