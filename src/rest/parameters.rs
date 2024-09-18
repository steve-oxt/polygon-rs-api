use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Parameter {
    Ticker(String),
    Date,
    Adjusted,
    Sort,
    Limit,
    Timespan,
    From,
    To,
    Multiplier,
    IncludeOTC,
    OptionsTicker,
    Order,
    ContractType,
    Timestamp,
    Sortv3,
    StrikePrice,
}

#[derive(Clone, Debug)]
pub struct ParameterRequirment {
    pub required: bool,
    pub parameter: Parameter,
   
}

impl ParameterRequirment {
    pub fn verify(&self) {

    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Parameters {
    pub api_key: String,
    pub ticker: Option<String>,
    pub multiplier: Option<u16>,
    pub timespan: Option<Timespan>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub adjusted: Option<bool>,
    pub sort: Option<Sort>,
    pub order: Option<Order>,
    pub sortv3: Option<Sortv3>,
    pub timestamp: Option<String>,
    pub limit: Option<u16>,
    pub date: Option<String>,
    pub verbose: Option<bool>,
    pub contract_type: Option<ContractType>,
    pub include_otc: Option<bool>,
    pub strike_price: Option<f64>,
    pub strike_price_from: Option<f64>,
    pub strike_price_to: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Display)]
pub enum ContractType {
    Call,
    Put,
    Other,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ContractStyle {
    American,
    European,
    Bermudan,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Timeframe {
    Delayed,
    RealTime,
    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Sort {
    Asc,
    Desc,
}
#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Order {
    Asc,
    Desc,
}
#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Sortv3 {
    Timestamp,
    Ticker,
    ExpirationDate,
    StrikePrice,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Timespan {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quater,
    Year,
}
