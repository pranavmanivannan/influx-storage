mod data;
mod receiver;
mod sender;

use std::{sync::mpsc, thread, time::Duration};

use data::MarketData;
use receiver::Receiver;
use sender::Sender;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sender = Sender::new(tx);

    let mut receiver = Receiver::new(
        rx,
        vec![],
        100,
        "Huobi".to_string(),
        "TradeDetails".to_string(),
    );

    let sender_handle = thread::spawn(move || loop {
        let data = MarketData {
            id: 1,
            ts: 2,
            tick: 3,
        };
        sender.send_data(&data);
        thread::sleep(Duration::from_millis(100));
    });

    let receiver_handle = thread::spawn(move || loop {
        receiver.receive_data();
    });

    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
}
