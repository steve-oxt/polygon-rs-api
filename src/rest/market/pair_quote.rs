use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct PairQuote {
    bbo_parameters: Parameters,
    bbo_url: String,
    to: String,
    from: String,
    pub request_id: String,
    pub quote: Quote,
    pub status: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub ask_price: f64,
    pub bid_price: f64,
    pub timestamp: i64,
    pub exchange: i64,
}

impl PairQuote {
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

impl Request for PairQuote {
    const VERSION: &'static str = "v1";
    const CALL: &'static str = "lastquote/currencies";
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
        if let Err(check) = self.check_parameters(&TickerTypes::forex()) {
            return Err(check);
        }
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
                    if let Some(ask_exchange) = last["exchange"].as_i64() {
                        self.quote.exchange = ask_exchange
                    }
                    if let Some(ask_price) = last["ask"].as_f64() {
                        self.quote.ask_price = ask_price
                    }
                    if let Some(bid_price) = last["bid"].as_f64() {
                        self.quote.bid_price = bid_price
                    }
                    if let Some(participant_timestamp) = last["timestamp"].as_i64() {
                        self.quote.timestamp = participant_timestamp
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
