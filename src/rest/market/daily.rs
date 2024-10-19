use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request, TickerTypes};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Daily {
    #[serde(skip_serializing)]
    daily_parameters: Parameters,
    #[serde(skip_serializing)]
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
        if let Err(check) = self.check_parameters(&TickerTypes::set(true, true, false, false, true))
        {
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
        match self.polygon_request() {
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
        };

        Ok(())
    }
}
