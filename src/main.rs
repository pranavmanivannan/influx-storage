mod sender;
mod receiver;
mod data;

use std::{thread, sync::mpsc, time::Duration};

use data::MarketData;
use receiver::Receiver;
use sender::Sender;

fn main() {

    let (tx, rx) = mpsc::channel();

    let sender = Sender { tx };

    let mut receiver = Receiver { rx, buffer: vec![], buffer_capacity: 100, database: "Huobi".to_string(), table: "TradeDetails".to_string() };
    
    let sender_handle = thread::spawn(move || {
        loop {
            let data = MarketData {id: 1, ts: 2, tick: 3};
            sender.send_data(data);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let receiver_handle = thread::spawn(move || {
        loop {
            let received = receiver.receive_data();
        }
    });


    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();

}