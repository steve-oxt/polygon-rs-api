use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request};
pub struct Attribute {
    pub regex: &'static str,
    pub name: &'static str,
}

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
    const ATTRIBUTES: &'static [&'static Attribute] = &[
        &Attribute {
            regex: "\"(status)\":(.*?)(,|})",
            name: "status",
            func: Box::new(Self::verify_ticker),
        },
        &Attribute {
            regex: "\"(symbol)\":(.*?)(,|})",
            name: "symbol",
        },
        &Attribute {
            regex: "\"(afterHours)\":(.*?)(,|})",
            name: "after_hours",
        },
        &Attribute {
            regex: "\"(close)\":(.*?)(,|})",
            name: "close",
        },
        &Attribute {
            regex: "\"(from)\":(.*?)(,|})",
            name: "from",
        },
        &Attribute {
            regex: "\"(high)\":(.*?)(,|})",
            name: "high",
        },
        &Attribute {
            regex: "\"(low)\":(.*?)(,|})",
            name: "low",
        },
        &Attribute {
            regex: "\"(open)\":(.*?)(,|})",
            name: "open",
        },
        &Attribute {
            regex: "\"(preMarket)\":(.*?)(,|})",
            name: "pre_market",
        },
        &Attribute {
            regex: "\"(volume)\":(.*?)(,|})",
            name: "volume",
        },
    ];
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
            match a.name {
                "status" => self.status = self.return_parsed_string(a.regex.to_string(), &res),
                "symbol" => self.symbol = self.return_parsed_string(a.regex.to_string(), &res),
                "after_hours" => {
                    self.after_hours = self.return_parsed_number(a.regex.to_string(), &res)
                }
                "close" => self.close = self.return_parsed_number(a.regex.to_string(), &res),
                "from" => self.from = self.return_parsed_string(a.regex.to_string(), &res),
                "high" => self.high = self.return_parsed_number(a.regex.to_string(), &res),
                "low" => self.low = self.return_parsed_number(a.regex.to_string(), &res),
                "open" => self.open = self.return_parsed_number(a.regex.to_string(), &res),
                "pre_market" => {
                    self.pre_market = self.return_parsed_number(a.regex.to_string(), &res)
                }
                "volume" => self.volume = self.return_parsed_number(a.regex.to_string(), &res),
                _ => (),
            }
        }
        Ok(())
    }
}
