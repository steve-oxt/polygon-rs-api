use crate::{
    rest::parameters::{TickerType, TickerTypes},
    ErrorCode, Order, Parameter, ParameterRequirment, Parameters, Request, Sortv3,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Indicies {
    indicies_parameters: Parameters,
    indicies_url: String,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Indicie>,
    pub status: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Indicie {
    pub timestamp: i64,
    pub market_status: String,
    pub name: String,
    pub session: Session,
    pub ticker: String,
    pub timeframe: String,
    pub ticker_type: TickerType,
    pub value: f64,
    pub error: String,
    pub message: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Session {
    pub change: f64,
    pub change_percent: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub previous_close: f64,
}

impl Indicies {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        tickers: Option<Vec<String>>,
        ticker_from: Option<String>,
        ticker_to: Option<String>,
        sort: Option<Sortv3>,
        limit: Option<u16>,
        order: Option<Order>,
    ) {
        let tickers = if ticker_from.is_some() || ticker_to.is_some() {
            None
        } else {
            tickers
        };
        self.indicies_parameters = Parameters {
            api_key: api_key,
            tickers: tickers,
            ticker_from: ticker_from,
            ticker_to: ticker_to,
            sortv3: sort,
            limit: limit,
            order: order,
            ..Parameters::default()
        }
    }
}

impl Request for Indicies {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "snapshot/indicies";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Tickers,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::TickerFrom,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::TickerTo,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Sortv3,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Limit,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Order,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.indicies_parameters
    }

    fn url(&mut self) -> &String {
        &self.indicies_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&&TickerTypes::indicies()) {
            return Err(check);
        }
        self.indicies_url = String::from(format!(
            "{}/{}/{}?{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            if let Some(tickers) = self.parameters().clone().tickers {
                let tickers_string = {
                    let mut t = String::new();
                    let mut first = true;
                    for ticker in tickers {
                        if first {
                            t = ticker.to_string();
                            first = false;
                            continue
                        }
                        t = format!("{},{}", t, ticker);
                    }
                    t
                };
                format!("ticker.any_of={}&", tickers_string)
            } else {
                "".to_string()
            },
            if let Some(tf) = self.parameters().clone().ticker_from {
                format!("ticker.gte={}&", tf)
            } else {
                "".to_string()
            },
            if let Some(tt) = self.parameters().clone().ticker_to {
                format!("ticker.lte={}&", tt)
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
                        let mut indicie = Indicie::default();
                        if let Some(last_updated) = result["last_updated"].as_i64() {
                            indicie.timestamp = last_updated
                        }
                        if let Some(market_status) = result["market_status"].as_str() {
                            indicie.market_status = market_status.to_string()
                        }
                        if let Some(name) = result["name"].as_str() {
                            indicie.name = name.to_string()
                        }
                        if let Some(session) = result["session"].as_object() {
                            if let Some(change) = session["change"].as_f64() {
                                indicie.session.change = change
                            }
                            if let Some(change_percent) = session["change_percent"].as_f64() {
                                indicie.session.change_percent = change_percent
                            }
                            if let Some(close) = session["close"].as_f64() {
                                indicie.session.close = close
                            }
                            if let Some(high) = session["high"].as_f64() {
                                indicie.session.high = high
                            }
                            if let Some(low) = session["low"].as_f64() {
                                indicie.session.low = low
                            }
                            if let Some(open) = session["open"].as_f64() {
                                indicie.session.open = open
                            }
                            if let Some(previous_close) = session["previous_close"].as_f64() {
                                indicie.session.previous_close = previous_close
                            }
                        }
                        if let Some(ticker) = result["ticker"].as_str() {
                            indicie.ticker = ticker.to_string()
                        }
                        if let Some(timeframe) = result["timeframe"].as_str() {
                            indicie.timeframe = timeframe.to_string()
                        }
                        if let Some(ticker_type) = result["ticker_type"].as_str() {
                            indicie.ticker_type = match ticker_type {
                                "stocks" => TickerType::Stocks,
                                "options" => TickerType::Options,
                                "indices" => TickerType::Indicies,
                                "forex" => TickerType::Forex,
                                "crypto" => TickerType::Crypto,
                                _ => TickerType::default(),
                            }
                        }
                        if let Some(value) = result["value"].as_f64() {
                            indicie.value = value
                        }
                        if let Some(error) = result["error"].as_str() {
                            indicie.error = error.to_string()
                        }
                        if let Some(message) = result["message"].as_str() {
                            indicie.message = message.to_string()
                        }
                        self.results.push(indicie);
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
