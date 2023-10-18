use std::sync::mpsc;
use crate::data;

pub struct Sender<T> 
where 
    T: data::Data
{
    pub tx: mpsc::Sender<T>,
}


impl<T> Sender<T>
where 
    T: data::Data
{

    pub fn send_data(&self, data: T) {
        match self.tx.send(data) { // need to conver to a clone //maybe make this unwrap_or_else too
            Ok(_) => {},
            Err(e) => {
                println!("Unable to send information to channel {:?}", e)
            }
        }
    }
}

