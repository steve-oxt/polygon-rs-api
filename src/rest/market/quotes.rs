use crate::{
    rest::parameters::TickerTypes, ErrorCode, Order, Parameter, ParameterRequirment, Parameters,
    Request, Sortv3,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Quotes {
    quotes_parameters: Parameters,
    quotes_url: String,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Quote>,
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub ask_exchange: i64,
    pub ask_price: f64,
    pub ask_size: i64,
    pub bid_exchange: i64,
    pub bid_price: f64,
    pub bid_size: i64,
    pub conditions: Vec<i64>,
    pub indicators: Vec<i64>,
    pub participant_timestamp: i64,
    pub sequence_number: i64,
    pub sip_timestamp: i64,
    pub tape: i64,
    pub trf_timestamp: i64,
}

impl Quotes {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        timestamp: Option<String>,
        from: Option<String>,
        to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) {
        let ts = if to.is_some() || from.is_some() {
            None
        } else {
            timestamp
        };
        self.quotes_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            timestamp: ts,
            from: from,
            to: to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        }
    }
}

impl Request for Quotes {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "quotes";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Timestamp,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::From,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::To,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Order,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Limit,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Sortv3,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.quotes_parameters
    }

    fn url(&mut self) -> &String {
        &self.quotes_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) =
            self.check_parameters(&TickerTypes::set(true, true, false, false, false))
        {
            return Err(check);
        }
        self.quotes_url = String::from(format!(
            "{}/{}/{}/{}?{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            if let Some(t) = self.parameters().clone().timestamp {
                format!("timestamp={}&", t)
            } else {
                "".to_string()
            },
            if let Some(tf) = self.parameters().clone().from {
                format!("timestamp.gte={}&", tf)
            } else {
                "".to_string()
            },
            if let Some(tt) = self.parameters().clone().to {
                format!("timestamp.lte={}&", tt)
            } else {
                "".to_string()
            },
            if let Some(o) = self.parameters().clone().order {
                format!("order={}&", o)
            } else {
                "".to_string()
            },
            if let Some(l) = self.parameters().clone().limit {
                format!("limit={}&", l)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().sortv3 {
                format!("sort={}&", s)
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
                if let Some(next_url) = response["next_url"].as_str() {
                    self.next_url = next_url.to_string()
                } else {
                    self.next_url = "".to_string()
                }
                if let Some(results) = response["results"].as_array() {
                    for result in results {
                        let mut quote = Quote::default();
                        if let Some(ask_exchange) = result["ask_exchange"].as_i64() {
                            quote.ask_exchange = ask_exchange
                        }
                        if let Some(ask_price) = result["ask_price"].as_f64() {
                            quote.ask_price = ask_price
                        }
                        if let Some(ask_size) = result["ask_size"].as_i64() {
                            quote.ask_size = ask_size
                        }
                        if let Some(bid_exchange) = result["bid_exchange"].as_i64() {
                            quote.bid_exchange = bid_exchange
                        }
                        if let Some(bid_price) = result["bid_price"].as_f64() {
                            quote.bid_price = bid_price
                        }
                        if let Some(bid_size) = result["bid_size"].as_i64() {
                            quote.bid_size = bid_size
                        }
                        if let Some(conditions) = result["conditions"].as_array() {
                            for condition in conditions {
                                if let Some(c) = condition.as_i64() {
                                    quote.conditions.push(c)
                                }
                            }
                        }
                        if let Some(indicators) = result["indicators"].as_array() {
                            for indicator in indicators {
                                if let Some(i) = indicator.as_i64() {
                                    quote.indicators.push(i)
                                }
                            }
                        }
                        if let Some(participant_timestamp) =
                            result["participant_timestamp"].as_i64()
                        {
                            quote.participant_timestamp = participant_timestamp
                        }
                        if let Some(sequence_number) = result["sequence_number"].as_i64() {
                            quote.sequence_number = sequence_number
                        }
                        if let Some(sip_timestamp) = result["sip_timestamp"].as_i64() {
                            quote.sip_timestamp = sip_timestamp
                        }
                        if let Some(tape) = result["tape"].as_i64() {
                            quote.tape = tape
                        }
                        if let Some(trf_timestamp) = result["trf_timestamp"].as_i64() {
                            quote.trf_timestamp = trf_timestamp
                        }
                        self.results.push(quote);
                    }
                }
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }
}
