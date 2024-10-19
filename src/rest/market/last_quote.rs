use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct LastQuote {
    last_quote_parameters: Parameters,
    last_quote_url: String,
    pub request_id: String,
    pub results: Quote,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub ask_price: f64,
    pub ask_size: i64,
    pub exchange: String,
    pub exchange_id: i64,
    pub conditions: Vec<i64>,
    pub trf_timestamp: i64,
    pub indicators: Vec<i64>,
    pub bid_price: f64,
    pub sequence_number: i64,
    pub bid_size: i64,
    pub sip_timestamp: i64,
    pub participant_timestamp: i64,
    pub tape: i64,
}

impl LastQuote {
    pub fn set_parameters(&mut self, api_key: String, ticker: String) {
        self.last_quote_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
    }
}

impl Request for LastQuote {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "last/nbbo";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];

    fn parameters(&self) -> &Parameters {
        &self.last_quote_parameters
    }

    fn url(&mut self) -> &String {
        &self.last_quote_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::stocks()) {
            return Err(check);
        }
        self.last_quote_url = String::from(format!(
            "{}/{}/{}/{}?apiKey={}",
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
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(result) = response["results"].as_object() {
                    if let Some(ask_price) = result["P"].as_f64() {
                        self.results.ask_price = ask_price
                    }
                    if let Some(ask_size) = result["S"].as_i64() {
                        self.results.ask_size = ask_size
                    }
                    if let Some(exchange) = result["T"].as_str() {
                        self.results.exchange = exchange.to_string()
                    }
                    if let Some(exchange_id) = result["x"].as_i64() {
                        self.results.exchange_id = exchange_id
                    }
                    if let Some(conditions) = result["c"].as_array() {
                        for condition in conditions {
                            if let Some(c) = condition.as_i64() {
                                self.results.conditions.push(c)
                            }
                        }
                    }
                    if let Some(trf_timestamp) = result["f"].as_i64() {
                        self.results.trf_timestamp = trf_timestamp
                    }
                    if let Some(indicators) = result["i"].as_array() {
                        for indicator in indicators {
                            if let Some(i) = indicator.as_i64() {
                                self.results.indicators.push(i)
                            }
                        }
                    }
                    if let Some(bid_price) = result["p"].as_f64() {
                        self.results.bid_price = bid_price
                    }
                    if let Some(sequence_number) = result["q"].as_i64() {
                        self.results.sequence_number = sequence_number
                    }
                    if let Some(bid_size) = result["s"].as_i64() {
                        self.results.bid_size = bid_size
                    }
                    if let Some(sip_timestamp) = result["t"].as_i64() {
                        self.results.sip_timestamp = sip_timestamp
                    }
                    if let Some(participant_timestamp) = result["y"].as_i64() {
                        self.results.participant_timestamp = participant_timestamp
                    }
                    if let Some(tape) = result["z"].as_i64() {
                        self.results.tape = tape
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
