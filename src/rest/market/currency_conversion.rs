use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct CurrencyConversion {
    bbo_parameters: Parameters,
    bbo_url: String,
    pub to: String,
    pub from: String,
    pub request_id: String,
    pub quote: Quote,
    pub status: String,
    pub symbol: String,
    pub initial_amount: f64,
    pub converted: f64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub ask_price: f64,
    pub bid_price: f64,
    pub timestamp: i64,
    pub exchange: i64,
}

impl CurrencyConversion {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        amount: Option<f64>,
        precision: Option<u8>,
    ) {
        self.to = ticker.clone();
        self.from = ticker.clone();
        self.bbo_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            amount: amount,
            precision: precision,
            ..Parameters::default()
        }
    }
}

impl Request for CurrencyConversion {
    const VERSION: &'static str = "v1";
    const CALL: &'static str = "conversion";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Amount,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Precision,
        },
    ];

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
            "{}/{}/{}/{}/{}?{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            from,
            to,
            if let Some(s) = self.parameters().clone().amount {
                format!("amount={}&", s)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().precision {
                format!("precision={}&", s)
            } else {
                "".to_string()
            },
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
                if let Some(to) = response["to"].as_str() {
                    self.to = to.to_string()
                }
                if let Some(from) = response["from"].as_str() {
                    self.symbol = from.to_string()
                }
                if let Some(converted) = response["converted"].as_f64() {
                    self.converted = converted
                }
                if let Some(initial_amount) = response["initial_amount"].as_f64() {
                    self.initial_amount = initial_amount
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
