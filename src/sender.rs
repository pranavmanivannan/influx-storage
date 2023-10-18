use crate::data;
use std::sync::mpsc;

pub struct Sender<T>
where
    T: data::Data,
{
    pub sender_endpoint: mpsc::Sender<T>,
}

impl<T> Sender<T>
where
    T: data::Data + Clone,
{
    pub fn new(endpoint: mpsc::Sender<T>) -> Sender<T> {
        Sender {
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
