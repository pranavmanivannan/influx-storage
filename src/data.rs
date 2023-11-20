/// A struct containing a DataEnum holding the type of data and data itself, as well as the exchange and channel
/// names. It also holds a timestamp for timeseries storage.
pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: ExchangeEnum,
    pub symbol_pair: SymbolEnum,
    pub channel: String,
    pub timestamp: i64,
}

/// An enum used to determine what the type of data in the DataPacket struct is. Each enum type holds a struct
/// containing that type of specific data.
pub enum DataEnum {
    MBP(MarketIncremental),
    RBA(RefreshBidAsk),
}

pub enum ExchangeEnum {
    Huobi, 
    Binance,
}

pub enum SymbolEnum {
    BTCUSD,
    ETHUSD,
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