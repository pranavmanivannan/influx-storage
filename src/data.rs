/// A struct containing a DataEnum holding the type of data and data itself, as well as the exchange and channel
/// names. It also holds a timestamp for timeseries storage.
pub struct DataPacket {
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
    pub timestamp: i64,
}

/// An enum used to determine what the type of data in the DataPacket struct is. Each enum type holds a struct
/// containing that type of specific data.
pub enum DataEnum {
    BBABinanceBTCData(BestBidAskDataBTCBinance),
    BBABinanceETHData(BestBidAskDataETHBinance),
    BBAHuobiBTCData(BestBidAskDataBTCHuobi),
    BBAHuobiETHData(BestBidAskDataETHHobi),
}

pub struct BestBidAskDataBTCBinance {
    pub bestask: f64,
    pub askamt: f64,
}

pub struct BestBidAskDataETHBinance {
    pub bestask: f64,
    pub askamt: f64,
}

pub struct BestBidAskDataBTCHuobi {
    pub bestask: f64,
    pub askamt: f64,
    pub bestbid: f64,
    pub bidamt: f64,
}

pub struct BestBidAskDataETHHobi {
    pub bestask: f64,
    pub askamt: f64,
    pub bestbid: f64,
    pub bidamt: f64,
}
