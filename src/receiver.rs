use std::sync::mpsc;
use hyper::Request;
use serde_json::json;

use crate::data;

pub struct Receiver<T> 
where 
    T: data::Data
{
    pub rx: mpsc::Receiver<T>,
    pub buffer: Vec<T>,
    pub buffer_capacity: usize,
    pub database: String,
    pub table: String
}


impl<T> Receiver<T> 
where 
    T: data::Data
{

    pub fn receive_data(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(data) => self.buffer.push(data),
                Err(e) => println!("Unable to receive data: {:?}", e),
            }

            if self.buffer.len() > self.buffer_capacity {
                self.upload_data();
            }
        }
    }

    fn upload_data(&mut self) {

        // code that uploads to aws and stuff
        let request = Request::builder()
            .method("POST")
            .uri("Amazon URI");

        self.buffer.clear();
    }
}