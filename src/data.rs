/// A struct containing a DataEnum, ExchangeEnum, and SymbolEnum for storage classificaiton. It also
/// holds the channel name within the exchange and the timestamp.
pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: ExchangeEnum,
    pub symbol_pair: SymbolEnum,
    pub channel: String,
    pub timestamp: i64,
}

/// An enum used to determine the type of data held within the sent DataPacket.
pub enum DataEnum {
    MBP(MarketIncremental),
    RBA(RefreshBidAsk),
}

/// An enum used to determine which exchange the DataPacket received information from.
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

/// An enum used to determine the symbol the DataPacket holds.
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

pub struct MarketIncremental {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
}

pub struct RefreshBidAsk {
    pub asks: Vec<(f64, f64)>, //price, amount
    pub bids: Vec<(f64, f64)>, //price, amount
}
