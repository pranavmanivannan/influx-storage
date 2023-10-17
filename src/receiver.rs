// need aws/http crates here
// need logging crates here, idt theres a built-in

use std::sync::mpsc;

use serde_json::json;

use crate::data;

pub struct Receiver<T> {
    pub rx: mpsc::Receiver<T>,
    pub buffer: Vec<T>,
    pub buffer_capacity: usize,
}


impl<Data> Receiver<Data> {

    fn receive_data(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(data) => {
                    // let json = json!(data).to_string();
                    // self.buffer.push(json)
                    self.buffer.push(data);
                },
                Err(e) => println!("Unable to receive data: {:?}", e),
            }

            if self.buffer.len() > self.buffer_capacity { // uploads data to aws if over buffer capacity
                self.upload_data();
            }
        }
    }

    fn upload_data(&mut self) {

        // code that uploads to aws and stuff

        self.buffer.clear(); // clear buffer for repeated use
    }
}