use std::sync::mpsc;

use crate::data;

pub struct Receiver<T>
where
    T: data::Data,
{
    pub receiver_endpoint: mpsc::Receiver<T>,
    pub buffer: Vec<T>,
    pub buffer_capacity: usize,
    pub database_name: String,
    pub table_name: String,
}

impl<T> Receiver<T>
where
    T: data::Data,
{
    pub fn new(
        endpoint: mpsc::Receiver<T>,
        buf: Vec<T>,
        buf_capacity: usize,
        db_name: String,
        tb_name: String,
    ) -> Receiver<T> {
        Receiver {
            receiver_endpoint: endpoint,
            buffer: buf,
            buffer_capacity: buf_capacity,
            database_name: db_name,
            table_name: tb_name,
        }
    }

    pub fn receive_data(&mut self) {
        loop {
            match self.receiver_endpoint.recv() {
                Ok(data) => self.buffer.push(data),
                Err(e) => println!("Unable to receive data: {:?}", e),
            }
            println!("Working"); // for testing
            if self.buffer.len() > self.buffer_capacity {
                self.upload_data();
            }
        }
    }

    fn upload_data(&mut self) {
        // user hyper.rs for the following steps:
        // 1) check if database exists, if not make one (use self.database)
        // 2) check if table in database exists, if not make one (use self.table)
        // 3) upload data to AWS

        self.buffer.clear();
    }
}
