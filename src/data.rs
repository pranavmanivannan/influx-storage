pub struct DataPacket {
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
    pub timestamp: i64,
}

///////////////////////////////////////////////

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
