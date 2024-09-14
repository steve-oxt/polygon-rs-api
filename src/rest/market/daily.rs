use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
use crate::rest::Regex;

pub struct Attribute {
    pub regex: &'static str,
    pub name: &'static str,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Daily {
    daily_parameters: Parameters,
    daily_url: String,
    pub after_hours: f64,
    pub close: f64,
    pub from: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub pre_market: f64,
    pub status: String,
    pub symbol: String,
    pub volume: f64,
}

impl Daily {
    const ATTRIBUTES: &'static [&'static Attribute] = &[&Attribute {
        regex: "\"(status)\":(\\\".*?\\\")",
        name: "status",
    }, &Attribute {
        regex: "\"(symbol)\":(\\\".*?\\\")",
        name: "symbol",
    }, &Attribute {
        regex: "\"(afterHours)\":(\\\".*?\\\")",
        name: "after_hours",
    },&Attribute {
        regex: "\"(close)\":(\\\".*?\\\")",
        name: "close",
    },&Attribute {
        regex: "\"(from)\":(\\\".*?\\\")",
        name: "from",
    },&Attribute {
        regex: "\"(high)\":(\\\".*?\\\")",
        name: "high",
    },&Attribute {
        regex: "\"(low)\":(\\\".*?\\\")",
        name: "low",
    },&Attribute {
        regex: "\"(open)\":(\\\".*?\\\")",
        name: "open",
    },&Attribute {
        regex: "\"(preMarket)\":(\\\".*?\\\")",
        name: "pre_market",
    },&Attribute {
        regex: "\"(volume)\":(\\\".*?\\\")",
        name: "volume",
    },];
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        date: String,
        adjusted: Option<bool>,
    ) {
        self.daily_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            date: Some(date),
            adjusted: adjusted,
            ..Parameters::default()
        }
    }
}



impl Request for Daily {
    const VERSION: &'static str = "v1";
    const CALL: &'static str = "open-close";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.daily_parameters
    }

    fn url(&mut self) -> &String {
        &self.daily_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters() {
            return Err(check);
        }
        self.daily_url = String::from(format!(
            "{}/{}/{}/{}/{}?{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().date.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        let res = match self.polygon_request_string() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        
        for a in Self::ATTRIBUTES {
            let v = Regex::new(a.regex).unwrap().find(&res).unwrap().as_str();
            match a.name {
                "status" => self.status = Regex::new("(?<=:\").*?(?=\")").unwrap().find(&v).unwrap().as_str().to_string(),
                "symbol" => self.symbol = v.to_string(),
                "after_hours" => self.after_hours = v.parse::<f64>().unwrap(),
                "close" => self.close = v.parse::<f64>().unwrap(),
                "from" => self.from = v.to_string(),
                "high" => self.high = v.parse::<f64>().unwrap(),
                "low" => self.low = v.parse::<f64>().unwrap(),
                "open" => self.open = v.parse::<f64>().unwrap(),
                "pre_market" => self.pre_market = v.parse::<f64>().unwrap(),
                "volume" => self.volume = v.parse::<f64>().unwrap(),
                _ => ()
            }
        }
        /*match self.polygon_request() {
            Ok(response) => {
                if let Some(after_hours) = response["afterHours"].as_f64() {
                    self.after_hours = after_hours
                }
                if let Some(close) = response["close"].as_f64() {
                    self.close = close
                }
                if let Some(from) = response["from"].as_str() {
                    self.from = from.to_string()
                }
                if let Some(high) = response["high"].as_f64() {
                    self.high = high
                }
                if let Some(low) = response["low"].as_f64() {
                    self.low = low
                }
                if let Some(open) = response["open"].as_f64() {
                    self.open = open
                }
                if let Some(pre_market) = response["preMarket"].as_f64() {
                    self.pre_market = pre_market
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(symbol) = response["symbol"].as_str() {
                    self.symbol = symbol.to_string()
                }
                if let Some(volume) = response["volume"].as_f64() {
                    self.volume = volume
                }
            }
            Err(e) => return Err(e),
        };*/

        Ok(())
    }
}
