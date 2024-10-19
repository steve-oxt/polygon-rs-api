use crate::{
    rest::parameters::{TickerType, TickerTypes},
    ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct TickerSnapshot {
    ticker_snapshot_url: String,
    ticker_snapshot_parameters: Parameters,
    ticker_type: TickerType,
    pub status: String,
    pub ticker: Ticker,
    pub request_id: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Ticker {
    pub day: Day,
    pub last_trade: LastTrade,
    pub last_quote: LastQuote,
    pub min: Min,
    pub previous_day: Day,
    pub ticker: String,
    pub todays_change: f64,
    pub todays_change_percent: f64,
    pub timestamp: i64,
    pub fair_market_value: f64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Day {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub volume_weighted_average_price: f64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct LastTrade {
    pub conditions: Vec<i64>,
    pub exchange_id: i64,
    pub price: f64,
    pub size: f64,
    pub id: String,
    pub timestamp: i64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct LastQuote {
    pub ask_price: f64,
    pub bid_price: f64,
    pub ask_size: i64,
    pub bid_size: i64,
    pub timestamp: i64,
    pub exchange_id: i64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Min {
    pub accumulated_volume: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub volume_weighted_average_price: f64,
    pub transactions: i64,
    pub timestamp: i64,
}

impl TickerSnapshot {
    pub fn set_parameters(&mut self, api_key: String, ticker: String, ticker_type: TickerType) {
        self.ticker_type = ticker_type;
        self.ticker_snapshot_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            ..Parameters::default()
        }
    }
}

impl Request for TickerSnapshot {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "snapshot/locale";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[&ParameterRequirment {
        required: true,
        parameter: Parameter::Ticker,
    }];

    fn parameters(&self) -> &Parameters {
        &self.ticker_snapshot_parameters
    }

    fn url(&mut self) -> &String {
        &self.ticker_snapshot_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::set(true, false, false, true, true))
        {
            return Err(check);
        }
        let locale = match self.ticker_type {
            TickerType::Options | TickerType::Indicies => {
                return Err(ErrorCode::TickerTypeeNotValidForAPICall)
            }
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
        };
        self.ticker_snapshot_url = String::from(format!(
            "{}/{}/{}/{}/markets/{}/{}/tickers/{}?apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            locale,
            self.ticker_type.to_string().to_lowercase(),
            self.parameters().clone().ticker.unwrap(),
            if let Some(s) = self.parameters().clone().include_otc {
                format!("include_otc={}&", s)
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
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(request_id_string) = response["request_id"].as_str() {
                    self.request_id = request_id_string.to_string()
                }
                if let Some(ticker) = response["ticker"].as_object() {
                    if let Some(day) = ticker["day"].as_object() {
                        if let Some(open) = day["o"].as_f64() {
                            self.ticker.day.open = open
                        }
                        if let Some(high) = day["h"].as_f64() {
                            self.ticker.day.high = high
                        }
                        if let Some(low) = day["l"].as_f64() {
                            self.ticker.day.low = low
                        }
                        if let Some(close) = day["c"].as_f64() {
                            self.ticker.day.close = close
                        }
                        if let Some(volume) = day["v"].as_f64() {
                            self.ticker.day.volume = volume
                        }
                        if let Some(volume_weighted_average_price) = day["vw"].as_f64() {
                            self.ticker.day.volume_weighted_average_price =
                                volume_weighted_average_price
                        }
                    }
                    if let Some(fair_market_value) = ticker["fairMarketValue"].as_f64() {
                        self.ticker.fair_market_value = fair_market_value
                    }
                    if let Some(last_trade) = ticker["lastTrade"].as_object() {
                        if let Some(conditions) = last_trade["c"].as_array() {
                            for c in conditions {
                                if let Some(condition) = c.as_i64() {
                                    self.ticker.last_trade.conditions.push(condition)
                                }
                            }
                        }
                        if let Some(id) = last_trade["i"].as_str() {
                            self.ticker.last_trade.id = id.to_string()
                        }
                        if let Some(price) = last_trade["p"].as_f64() {
                            self.ticker.last_trade.price = price
                        }
                        if let Some(size) = last_trade["s"].as_f64() {
                            self.ticker.last_trade.size = size
                        }
                        if let Some(timestamp) = last_trade["t"].as_i64() {
                            self.ticker.last_trade.timestamp = timestamp
                        }
                        if let Some(exchange_id) = last_trade["x"].as_i64() {
                            self.ticker.last_trade.exchange_id = exchange_id
                        }
                    }
                    if let Some(last_quote) = ticker["lastQuote"].as_object() {
                        if let Some(ask_price) = last_quote["a"].as_f64() {
                            self.ticker.last_quote.ask_price = ask_price
                        }
                        if let Some(bid_price) = last_quote["b"].as_f64() {
                            self.ticker.last_quote.bid_price = bid_price
                        }
                        if let Some(bid_price) = last_quote["p"].as_f64() {
                            self.ticker.last_quote.bid_price = bid_price
                        }
                        if let Some(ask_price) = last_quote["P"].as_f64() {
                            self.ticker.last_quote.ask_price = ask_price
                        }
                        if let Some(bid_size) = last_quote["a"].as_i64() {
                            self.ticker.last_quote.bid_size = bid_size
                        }
                        if let Some(ask_size) = last_quote["A"].as_i64() {
                            self.ticker.last_quote.ask_size = ask_size
                        }
                        if let Some(timestamp) = last_quote["t"].as_i64() {
                            self.ticker.last_quote.timestamp = timestamp
                        }
                        if let Some(exchange_id) = last_quote["x"].as_i64() {
                            self.ticker.last_quote.exchange_id = exchange_id
                        }
                    }
                    if let Some(min) = ticker["min"].as_object() {
                        if let Some(accumulated_volume) = min["av"].as_i64() {
                            self.ticker.min.accumulated_volume = accumulated_volume
                        }
                        if let Some(open) = min["o"].as_f64() {
                            self.ticker.min.open = open
                        }
                        if let Some(high) = min["h"].as_f64() {
                            self.ticker.min.high = high
                        }
                        if let Some(low) = min["l"].as_f64() {
                            self.ticker.min.low = low
                        }
                        if let Some(close) = min["c"].as_f64() {
                            self.ticker.min.close = close
                        }
                        if let Some(volume) = min["v"].as_f64() {
                            self.ticker.min.volume = volume
                        }
                        if let Some(volume_weighted_average_price) = min["vw"].as_f64() {
                            self.ticker.min.volume_weighted_average_price =
                                volume_weighted_average_price
                        }
                        if let Some(transactions) = min["n"].as_i64() {
                            self.ticker.min.transactions = transactions
                        }
                        if let Some(timestamp) = min["t"].as_i64() {
                            self.ticker.min.timestamp = timestamp
                        }
                    }
                    if let Some(previous_day) = ticker["prevDay"].as_object() {
                        if let Some(open) = previous_day["o"].as_f64() {
                            self.ticker.previous_day.open = open
                        }
                        if let Some(high) = previous_day["h"].as_f64() {
                            self.ticker.previous_day.high = high
                        }
                        if let Some(low) = previous_day["l"].as_f64() {
                            self.ticker.previous_day.low = low
                        }
                        if let Some(close) = previous_day["c"].as_f64() {
                            self.ticker.previous_day.close = close
                        }
                        if let Some(volume) = previous_day["v"].as_f64() {
                            self.ticker.previous_day.volume = volume
                        }
                        if let Some(volume_weighted_average_price) = previous_day["vw"].as_f64() {
                            self.ticker.previous_day.volume_weighted_average_price =
                                volume_weighted_average_price
                        }
                    }
                    if let Some(ticker) = ticker["t"].as_str() {
                        self.ticker.ticker = ticker.to_string()
                    }
                    if let Some(todays_change) = ticker["todaysChange"].as_f64() {
                        self.ticker.todays_change = todays_change
                    }
                    if let Some(todays_change_percent) = ticker["todaysChangePercent"].as_f64() {
                        self.ticker.todays_change_percent = todays_change_percent
                    }
                    if let Some(timestamp) = ticker["updated"].as_i64() {
                        self.ticker.timestamp = timestamp
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
