use crate::{
    rest::parameters::TickerTypes, ErrorCode, Order, Parameter, ParameterRequirment, Parameters,
    Request, Sortv3,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Trades {
    trades_parameters: Parameters,
    trades_url: String,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Trade>,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub conditions: Vec<i64>,
    pub correction: i64,
    pub exchange: i64,
    pub id: i64,
    pub participant_timestamp: i64,
    pub price: f64,
    pub sequence_number: i64,
    pub sip_timestamp: i64,
    pub size: i64,
    pub tape: i64,
    pub trf_timestamp: i64,
    pub trf_id: i64,
}

impl Trades {
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
        self.trades_parameters = Parameters {
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

impl Request for Trades {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "trades";
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
        &self.trades_parameters
    }

    fn url(&mut self) -> &String {
        &self.trades_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::set(true, true, false, false, true))
        {
            return Err(check);
        }
        self.trades_url = String::from(format!(
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
                        let mut trade = Trade::default();
                        if let Some(correction) = result["correction"].as_i64() {
                            trade.correction = correction
                        }
                        if let Some(exchange) = result["exchange"].as_i64() {
                            trade.exchange = exchange
                        }
                        if let Some(id) = result["id"].as_i64() {
                            trade.id = id
                        }
                        if let Some(participant_timestamp) =
                            result["participant_timestamp"].as_i64()
                        {
                            trade.participant_timestamp = participant_timestamp
                        }
                        if let Some(price) = result["price"].as_f64() {
                            trade.price = price
                        }
                        if let Some(sequence_number) = result["sequence_number"].as_i64() {
                            trade.sequence_number = sequence_number
                        }
                        if let Some(sip_timestamp) = result["sip_timestamp"].as_i64() {
                            trade.sip_timestamp = sip_timestamp
                        }
                        if let Some(size) = result["size"].as_i64() {
                            trade.size = size
                        }
                        if let Some(tape) = result["tape"].as_i64() {
                            trade.tape = tape
                        }
                        if let Some(trf_timestamp) = result["trf_timestamp"].as_i64() {
                            trade.trf_timestamp = trf_timestamp
                        }
                        if let Some(trf_id) = result["trf_id"].as_i64() {
                            trade.trf_id = trf_id
                        }
                        if let Some(conditions) = result["conditions"].as_array() {
                            for condition in conditions {
                                if let Some(c) = condition.as_i64() {
                                    trade.conditions.push(c)
                                }
                            }
                        }
                        self.results.push(trade);
                    }
                }
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }
}
