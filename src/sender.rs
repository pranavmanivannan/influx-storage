use std::sync::mpsc;
use crate::data::Data;

pub struct Sender {
    pub tx: mpsc::Sender<Data>,
}


impl Sender{

    fn send_data(&self, data: &Data) {
        let data = data.clone();
        // let json = serde_json::to_string(data).unwrap(); // probably make an unwrap or else method here
        match self.tx.send(data) { //maybe make this unwrap_or_else too
            Ok(_) => {},
            Err(e) => {
                println!("Unable to send information to channel {:?}", e)
            }
        }
    }
}