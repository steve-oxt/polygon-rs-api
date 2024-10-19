use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct LastTrade {
    last_trade_parameters: Parameters,
    last_trade_url: String,
    pub request_id: String,
    pub results: Trade,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub exchange: String,
    pub conditions: Vec<i64>,
    pub trade_correction: i64,
    pub trf_timestamp: i64,
    pub trade_id: String,
    pub price: f64,
    pub sequence_number: i64,
    pub trf_id: i64,
    pub size: i64,
    pub sip_timestamp: i64,
    pub exchange_id: i64,
    pub participant_timestamp: i64,
    pub tape: i64,
}

impl LastTrade {
    pub fn set_parameters(&mut self, api_key: String, ticker: String) {
        self.last_trade_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
    }
}

impl Request for LastTrade {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "last/trade";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];

    fn parameters(&self) -> &Parameters {
        &self.last_trade_parameters
    }

    fn url(&mut self) -> &String {
        &self.last_trade_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) =
            self.check_parameters(&TickerTypes::set(true, true, false, false, false))
        {
            return Err(check);
        }
        self.last_trade_url = String::from(format!(
            "{}/{}/{}/{}apiKey={}",
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
                    if let Some(trade_correction) = result["e"].as_i64() {
                        self.results.trade_correction = trade_correction
                    }
                    if let Some(trf_timestamp) = result["f"].as_i64() {
                        self.results.trf_timestamp = trf_timestamp
                    }
                    if let Some(trade_id) = result["i"].as_str() {
                        self.results.trade_id = trade_id.to_string()
                    }
                    if let Some(bid_price) = result["p"].as_f64() {
                        self.results.price = bid_price
                    }
                    if let Some(sequence_number) = result["q"].as_i64() {
                        self.results.sequence_number = sequence_number
                    }
                    if let Some(trf_id) = result["r"].as_i64() {
                        self.results.trf_id = trf_id
                    }
                    if let Some(bid_size) = result["s"].as_i64() {
                        self.results.size = bid_size
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
