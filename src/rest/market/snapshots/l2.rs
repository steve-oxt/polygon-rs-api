use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};
use std::collections::HashMap;

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct L2 {
    l2_parameters: Parameters,
    l2_url: String,
    pub data: Data,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Data {
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
    pub bid_count: f64,
    pub ask_count: f64,
    pub timestamp: i64,
    pub spread: f64,
    pub ticker: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Asks {
    pub price: f64,
    pub size: HashMap<String, f64>,
}
#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Bids {
    pub price: f64,
    pub size: HashMap<String, f64>,
}

impl L2 {
    pub fn set_parameters(&mut self, api_key: String, ticker: String) {
        self.l2_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
    }
}

impl Request for L2 {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "snapshot/locale/global/markets/crypto/tickers";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];

    fn parameters(&self) -> &Parameters {
        &self.l2_parameters
    }

    fn url(&mut self) -> &String {
        &self.l2_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&&TickerTypes::crypto()) {
            return Err(check);
        }
        self.l2_url = String::from(format!(
            "{}/{}/{}/{}/book?apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        match self.polygon_request() {
            Ok(response) => {
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }

                if let Some(data) = response["data"].as_object() {
                    if let Some(bid_count) = data["bid_count"].as_f64() {
                        self.data.bid_count = bid_count
                    }
                    if let Some(ask_count) = data["ask_count"].as_f64() {
                        self.data.ask_count = ask_count
                    }
                    if let Some(bids) = data["bids"].as_array() {
                        for bid in bids {
                            let mut b = Bids::default();
                            if let Some(price) = bid["price"].as_f64() {
                                b.price = price
                            }
                            if let Some(size) = bid["size"].as_object() {
                                for (k, v) in size {
                                    b.size.insert(k.to_string(), v.as_f64().unwrap());
                                }
                            }
                            self.data.bids.push(b);
                        }
                    }
                    if let Some(asks) = data["asks"].as_array() {
                        for ask in asks {
                            let mut a = Asks::default();
                            if let Some(price) = ask["price"].as_f64() {
                                a.price = price
                            }
                            if let Some(size) = ask["size"].as_object() {
                                for (k, v) in size {
                                    a.size.insert(k.to_string(), v.as_f64().unwrap());
                                }
                            }
                            self.data.asks.push(a)
                        }
                    }
                    if let Some(timestamp) = data["timestamp"].as_i64() {
                        self.data.timestamp = timestamp
                    }
                    if let Some(spread) = data["spread"].as_f64() {
                        self.data.spread = spread
                    }
                    if let Some(ticker) = data["ticker"].as_str() {
                        self.data.ticker = ticker.to_string()
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
