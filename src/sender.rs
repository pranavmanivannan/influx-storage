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
    T: data::Data,
{
    pub fn new(endpoint: mpsc::Sender<T>) -> Sender<T> {
        Sender {
            sender_endpoint: endpoint,
        }
    }

    pub fn send_data(&self, data: T) {
        match self.sender_endpoint.send(data) {
            // need to conver to a clone //maybe make this unwrap_or_else too
            Ok(_) => {}
            Err(e) => {
                println!("Unable to send information to channel {:?}", e)
            }
        }
    }
}
