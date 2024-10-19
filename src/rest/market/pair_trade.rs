use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct PairTrade {
    bbo_parameters: Parameters,
    bbo_url: String,
    to: String,
    from: String,
    pub request_id: String,
    pub trade: Trade,
    pub status: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub conditions: Vec<i64>,
    pub price: f64,
    pub size: f64,
    pub timestamp: i64,
    pub exchange: i64,
}

impl PairTrade {
    pub fn set_parameters(&mut self, api_key: String, ticker: String) {
        self.to = ticker.clone();
        self.from = ticker.clone();
        self.bbo_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
    }
}

impl Request for PairTrade {
    const VERSION: &'static str = "v1";
    const CALL: &'static str = "last/crypto";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];

    fn parameters(&self) -> &Parameters {
        &self.bbo_parameters
    }

    fn url(&mut self) -> &String {
        &self.bbo_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&&TickerTypes::crypto()) {
            return Err(check);
        }
        //Need a different method to extract to and from as Crypto can be different lengths
        let from = self.from[2..4].to_string();
        let to = self.to[5..7].to_string();
        self.bbo_url = String::from(format!(
            "{}/{}/{}/{}/{}?apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            from,
            to,
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        match self.polygon_request() {
            Ok(response) => {
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(symbol) = response["symbol"].as_str() {
                    self.symbol = symbol.to_string()
                }
                if let Some(last) = response["last"].as_object() {
                    if let Some(conditions) = last["conditions"].as_array() {
                        for condition in conditions {
                            if let Some(c) = condition.as_i64() {
                                self.trade.conditions.push(c)
                            }
                        }
                    }
                    if let Some(ask_exchange) = last["exchange"].as_i64() {
                        self.trade.exchange = ask_exchange
                    }
                    if let Some(ask_price) = last["price"].as_f64() {
                        self.trade.price = ask_price
                    }
                    if let Some(bid_price) = last["size"].as_f64() {
                        self.trade.size = bid_price
                    }
                    if let Some(participant_timestamp) = last["timestamp"].as_i64() {
                        self.trade.timestamp = participant_timestamp
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
