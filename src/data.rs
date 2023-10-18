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
