///TODO: trade detail
pub enum ExchangeEnum {
    Huobi,
    Binance,
}

impl ExchangeEnum {
    pub fn as_str(&self) -> &str {
        match *self {
            ExchangeEnum::Huobi => "Houbi",
            ExchangeEnum::Binance => "Binance",
        }
    }
}

pub enum SymbolEnum {
    BTCUSD,
    ETHUSD,
}

impl SymbolEnum {
    pub fn as_str(&self) -> &str {
        match *self {
            SymbolEnum::BTCUSD => "BTCUSD",
            SymbolEnum::ETHUSD => "ETHUSD",
        }
    }
}

pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: ExchangeEnum,
    pub symbol_pair: SymbolEnum,
    pub channel: String,
    pub timestamp: i64,
}

pub enum DataEnum {
    MBP(MarketIncremental),
    RBA(RefreshBidAsk),
}

pub struct MarketIncremental {
    pub bestask: f64,
    pub askamount: f64,
    pub bestbid: f64,
    pub bidamount: f64,
}

pub struct RefreshBidAsk {
    pub asks: Vec<(f64, f64)>, //price, amount
    pub bids: Vec<(f64, f64)>, //price, amount
}
