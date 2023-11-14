use chrono::DateTime;
use chrono::Utc;
use influxdb::InfluxDbWriteable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, InfluxDbWriteable)]
pub struct AWSData {
    #[influxdb(tag)]
    pub exchange: String,
    pub json: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct DataPacket {
    pub temp_best_ask: String,
    pub temp_ask_amt: String,
    pub data: DataEnum,
    pub exchange: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize)]
pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}

#[derive(Serialize, Deserialize)]
pub struct MessageType1 {
    pub data: String,
    pub best_ask: f64,
    pub ask_amt: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MessageType2 {
    pub best_ask: String,
}
