use std::sync::mpsc;
use crate::data;

pub struct Sender<T> {
    pub tx: mpsc::Sender<T>,
}


impl<T> Sender<T>{

    fn send_data(&self, data: &T) {
        let data_clone = data.clone();
        match self.tx.send(data) { // error here need to fix clone //maybe make this unwrap_or_else too
            Ok(_) => {},
            Err(e) => {
                println!("Unable to send information to channel {:?}", e)
            }
        }
    }
}