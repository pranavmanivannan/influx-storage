use serde::{Deserialize, Serialize};

// need to figure out how to get rid of this trait and filter directly in receiver
pub trait Data {}

impl Data for MarketData {}
impl Data for TradeDetail {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub id: i32,
    pub ts: i32,
    pub tick: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TradeDetail {
    pub id: i32,
    pub ts: i32,
    pub tick: i32,
}

pub struct DataPacket {
    pub TempBestAsk: String,
    pub TempAskAmt: String,
    //change back to dataenum
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
}

pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}

pub struct MessageType1 {
    pub data: String,
    pub BestAsk: f64,
    pub AskAmt: f64,
}

pub struct MessageType2 {
    pub bestask: String,
}
