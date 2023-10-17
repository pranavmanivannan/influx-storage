use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketData {
    id: i32,
    ts: i32,
    tick: i32,
    
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TradeDetail {
    id: i32,
    ts: i32,
    tick: i32,
    
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Data {
    MarketData(MarketData),
    TradeDetail(TradeDetail),
}