use crate::security::{Secuirty, stocks::Stocks, options::Options, indices::Indices, forex::Forex, crypto::Crypto};
use crate::call::Call;
use crate::polygon::{timespan::Timespan, sort::Sort};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Polygon {
    pub security: Option<Secuirty>,
    pub call: Option<Call>,
    pub api_key: Option<String>,
    pub ticker: Option<String>,
    pub multiplier: Option<u16>,
    pub timespan: Option<Timespan>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub adjusted: Option<bool>,
    pub sort: Option<Sort>,
    pub limit: Option<u16>,
    pub date: Option<String>,
    pub verbose: Option<bool>,
}

impl Polygon {
    pub fn polygon(
        security: Option<Secuirty>,
        call: Option<Call>,
        api_key: Option<String>,
        ticker: Option<String>,
        multiplier: Option<u16>,
        timespan: Option<Timespan>,
        from: Option<String>,
        to: Option<String>,
        adjusted: Option<bool>,
        sort: Option<Sort>,
        limit: Option<u16>,
        date: Option<String>,
        verbose: Option<bool>,
    ) -> Polygon {
        Polygon {
            security,
            call,
            api_key,
            ticker,
            multiplier,
            timespan,
            from,
            to,
            adjusted,
            sort,
            limit,
            date,
            verbose
        }
    }

    pub fn request(&self) -> Result<Call, Box<dyn Error>> {
        match &self.security {
            Some(v) => match v {
                Secuirty::Stocks(_) => Stocks::request(&self.clone()),
                Secuirty::Options(_) => Options::request(&self.clone()),
                Secuirty::Indices(_) => Indices::request(&self.clone()),
                Secuirty::Forex(_) => Forex::request(&self.clone()),
                Secuirty::Crypto(_) => Crypto::request(&self.clone()),
            },
            None => panic!("There is either no security type set"),
        }
    }
}
