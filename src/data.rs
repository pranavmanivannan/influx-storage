use serde::{Deserialize, Serialize};

pub struct DataPacket {
    pub temp_best_ask: String,
    pub temp_ask_amt: String,
    //change back to dataenum
    pub data: DataEnum,
    pub exchange: String,
    pub channel: String,
}

pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}

pub struct MessageType1 {
    pub data: String,
    pub best_ask: f64,
    pub ask_amt: f64,
}

pub struct MessageType2 {
    pub best_ask: String,
}
