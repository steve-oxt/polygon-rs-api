use crate::{
    rest::parameters::{Sortv3, TickerTypes},
    ContractStyle, ContractType, ErrorCode, Order, Parameter, ParameterRequirment, Parameters,
    Request, Timeframe,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Chain {
    chain_parameters: Parameters,
    chain_url: String,
    pub request_id: String,
    pub next_url: String,
    pub results: Vec<Contract>,
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Contract {
    pub break_even_price: f64,
    pub day: Day,
    pub details: Details,
    pub fair_market_value: f64,
    pub greeks: Greeks,
    pub implied_volatility: f64,
    pub quote: Quote,
    pub trade: Trade,
    pub open_interest: i64,
    pub underlying_asset: UnderlyingAsset,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Day {
    pub change: f64,
    pub change_percent: f64,
    pub close: f64,
    pub high: f64,
    pub last_updated: i64,
    pub low: f64,
    pub open: f64,
    pub previous_close: f64,
    pub volume: i64,
    pub volume_weighted_average_price: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Details {
    pub contract_type: ContractType,
    pub contract_style: ContractStyle,
    pub expiration_date: String,
    pub shares_per_contract: i64,
    pub strike_price: f64,
    pub ticker: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Quote {
    pub bid: f64,
    pub bid_size: i64,
    pub ask: f64,
    pub ask_size: i64,
    pub bid_exchange_id: i64,
    pub ask_exchange_id: i64,
    pub last_updated: i64,
    pub mid_point: f64,
    pub timeframe: Timeframe,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Trade {
    pub conditions: Vec<i64>,
    pub exchange_id: i64,
    pub price: f64,
    pub sip_timestamp: i64,
    pub size: i64,
    pub timeframe: Timeframe,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct UnderlyingAsset {
    pub change_to_break_even: f64,
    pub last_updated: i64,
    pub price: f64,
    pub ticker: String,
    pub timeframe: Timeframe,
    pub value: f64,
}

impl Chain {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        underlying_asset: String,
        date: Option<String>,
        from: Option<String>,
        to: Option<String>,
        strike_price: Option<f64>,
        strike_price_from: Option<f64>,
        strike_price_to: Option<f64>,
        contract_type: Option<ContractType>,
        order: Option<Order>,
        limit: Option<u16>,
        sort: Option<Sortv3>,
    ) {
        let ts = if from.is_some() || from.is_some() {
            None
        } else {
            date
        };
        let sp = if strike_price_from.is_some() || strike_price_to.is_some() {
            None
        } else {
            strike_price
        };
        self.chain_parameters = Parameters {
            api_key: api_key,
            underlying_asset: Some(underlying_asset),
            date: ts,
            from: from,
            to: to,
            contract_type: contract_type,
            order: order,
            limit: limit,
            sortv3: sort,
            strike_price: sp,
            strike_price_from: strike_price_from,
            strike_price_to: strike_price_to,
            ..Parameters::default()
        }
    }
}

impl Request for Chain {
    const VERSION: &'static str = "v3";
    const CALL: &'static str = "snapshot/options";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::UnderlyingAsset,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::StrikePrice,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::StrikePriceFrom,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::StrikePriceTo,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::To,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::From,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::ContractType,
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
        &self.chain_parameters
    }

    fn url(&mut self) -> &String {
        &self.chain_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::options()) {
            return Err(check);
        }
        self.chain_url = String::from(format!(
            "{}/{}/{}/{}?{}{}{}{}{}{}{}{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().underlying_asset.unwrap(),
            if let Some(strike_price) = self.parameters().clone().strike_price {
                format!("strike_price={}&", strike_price)
            } else {
                "".to_string()
            },
            if let Some(strike_price_from) = self.parameters().clone().strike_price_from {
                format!("strike_price.gte={}&", strike_price_from)
            } else {
                "".to_string()
            },
            if let Some(strike_price_to) = self.parameters().clone().strike_price_to {
                format!("strike_price.lte={}&", strike_price_to)
            } else {
                "".to_string()
            },
            if let Some(date) = self.parameters().clone().date {
                format!("expiration_date={}&", date)
            } else {
                "".to_string()
            },
            if let Some(from) = self.parameters().clone().from {
                format!("expiration_date.gte={}&", from)
            } else {
                "".to_string()
            },
            if let Some(to) = self.parameters().clone().to {
                format!("expiration_date.lte={}&", to)
            } else {
                "".to_string()
            },
            if let Some(contract_type) = self.parameters().clone().contract_type {
                format!("contract_type={}&", contract_type)
            } else {
                "".to_string()
            },
            if let Some(order) = self.parameters().clone().order {
                format!("order={}&", order)
            } else {
                "".to_string()
            },
            if let Some(limit) = self.parameters().clone().limit {
                format!("limit={}&", limit)
            } else {
                "".to_string()
            },
            if let Some(sort) = self.parameters().clone().sortv3 {
                format!("sort={}&", sort)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        ));
        Ok(())
    }

    //This whole function is way to nested and should be rewritten in parts
    fn request(&mut self) -> Result<(), ErrorCode> {
        match self.polygon_request() {
            Ok(response) => {
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(next_url) = response["next_url"].as_str() {
                    self.next_url = next_url.to_string()
                } else {
                    self.next_url = "".to_string()
                }
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(results) = response["results"].as_array() {
                    for result in results {
                        let mut contract = Contract::default();
                        if let Some(break_even_price) = result["break_even_price"].as_f64() {
                            contract.break_even_price = break_even_price
                        }
                        if let Some(day) = result["day"].as_object() {
                            for key in day.keys() {
                                match key.as_str() {
                                    "change" => {
                                        if let Some(change) = day["change"].as_f64() {
                                            contract.day.change = change
                                        }
                                    }
                                    "change_percent" => {
                                        if let Some(change_percent) = day["change_percent"].as_f64()
                                        {
                                            contract.day.change_percent = change_percent
                                        }
                                    }
                                    "close" => {
                                        if let Some(close) = day["close"].as_f64() {
                                            contract.day.close = close
                                        }
                                    }
                                    "high" => {
                                        if let Some(high) = day["high"].as_f64() {
                                            contract.day.high = high
                                        }
                                    }
                                    "last_updated" => {
                                        if let Some(last_updated) = day["last_updated"].as_i64() {
                                            contract.day.last_updated = last_updated
                                        }
                                    }
                                    "low" => {
                                        if let Some(low) = day["low"].as_f64() {
                                            contract.day.low = low
                                        }
                                    }
                                    "open" => {
                                        if let Some(open) = day["open"].as_f64() {
                                            contract.day.open = open
                                        }
                                    }
                                    "previous_close" => {
                                        if let Some(previous_close) = day["previous_close"].as_f64()
                                        {
                                            contract.day.previous_close = previous_close
                                        }
                                    }
                                    "volume" => {
                                        if let Some(volume) = day["volume"].as_i64() {
                                            contract.day.volume = volume
                                        }
                                    }
                                    "volume_weighted_average_price" => {
                                        if let Some(volume_weighted_average_price) =
                                            day["volume_weighted_average_price"].as_f64()
                                        {
                                            contract.day.volume_weighted_average_price =
                                                volume_weighted_average_price
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        if let Some(details) = result["details"].as_object() {
                            for key in details.keys() {
                                match key.as_str() {
                                    "contract_type" => {
                                        if let Some(contract_type) =
                                            details["contract_type"].as_str()
                                        {
                                            contract.details.contract_type = match contract_type {
                                                "call" => ContractType::Call,
                                                "put" => ContractType::Put,
                                                _ => ContractType::Unknown,
                                            }
                                        }
                                    }
                                    "contract_style" => {
                                        if let Some(contract_style) =
                                            details["contract_style"].as_str()
                                        {
                                            contract.details.contract_style = match contract_style {
                                                "american" => ContractStyle::American,
                                                "european" => ContractStyle::European,
                                                "bermudan" => ContractStyle::Bermudan,
                                                _ => ContractStyle::Unknown,
                                            }
                                        }
                                    }
                                    "expiration_date" => {
                                        if let Some(expiration_date) =
                                            details["expiration_date"].as_str()
                                        {
                                            contract.details.expiration_date =
                                                expiration_date.to_string()
                                        }
                                    }
                                    "shares_per_contract" => {
                                        if let Some(shares_per_contract) =
                                            details["shares_per_contract"].as_i64()
                                        {
                                            contract.details.shares_per_contract =
                                                shares_per_contract
                                        }
                                    }
                                    "strike_price" => {
                                        if let Some(strike_price) = details["strike_price"].as_f64()
                                        {
                                            contract.details.strike_price = strike_price
                                        }
                                    }
                                    "ticker" => {
                                        if let Some(ticker) = details["ticker"].as_str() {
                                            contract.details.ticker = ticker.to_string()
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        if let Some(fair_market_value) = result["fair_market_value"].as_f64() {
                            contract.fair_market_value = fair_market_value
                        }
                        if let Some(greeks) = result["greeks"].as_object() {
                            for key in greeks.keys() {
                                match key.as_str() {
                                    "delta" => {
                                        if let Some(delta) = greeks["delta"].as_f64() {
                                            contract.greeks.delta = delta
                                        }
                                    }
                                    "gamma" => {
                                        if let Some(gamma) = greeks["gamma"].as_f64() {
                                            contract.greeks.gamma = gamma
                                        }
                                    }
                                    "theta" => {
                                        if let Some(theta) = greeks["theta"].as_f64() {
                                            contract.greeks.theta = theta
                                        }
                                    }
                                    "vega" => {
                                        if let Some(vega) = greeks["vega"].as_f64() {
                                            contract.greeks.vega = vega
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        if let Some(implied_volatility) = result["implied_volatility"].as_f64() {
                            contract.implied_volatility = implied_volatility
                        }
                        if let Some(quote) = result["quote"].as_object() {
                            for key in quote.keys() {
                                match key.as_str() {
                                    "bid" => {
                                        if let Some(bid) = quote["bid"].as_f64() {
                                            contract.quote.bid = bid
                                        }
                                    }
                                    "bid_size" => {
                                        if let Some(bid_size) = quote["bid_size"].as_i64() {
                                            contract.quote.bid_size = bid_size
                                        }
                                    }
                                    "ask" => {
                                        if let Some(ask) = quote["ask"].as_f64() {
                                            contract.quote.ask = ask
                                        }
                                    }
                                    "ask_size" => {
                                        if let Some(ask_size) = quote["ask_size"].as_i64() {
                                            contract.quote.ask_size = ask_size
                                        }
                                    }
                                    "bid_exchange_id" => {
                                        if let Some(bid_exchange_id) =
                                            quote["bid_exchange_id"].as_i64()
                                        {
                                            contract.quote.bid_exchange_id = bid_exchange_id
                                        }
                                    }
                                    "ask_exchange_id" => {
                                        if let Some(ask_exchange_id) =
                                            quote["ask_exchange_id"].as_i64()
                                        {
                                            contract.quote.ask_exchange_id = ask_exchange_id
                                        }
                                    }
                                    "last_updated" => {
                                        if let Some(last_updated) = quote["last_updated"].as_i64() {
                                            contract.quote.last_updated = last_updated
                                        }
                                    }
                                    "mid_point" => {
                                        if let Some(mid_point) = quote["mid_point"].as_f64() {
                                            contract.quote.mid_point = mid_point
                                        }
                                    }
                                    "timeframe" => {
                                        if let Some(timeframe) = quote["timeframe"].as_str() {
                                            contract.quote.timeframe = match timeframe {
                                                "DELAYED" => Timeframe::Delayed,
                                                "REAL-TIME" => Timeframe::RealTime,
                                                _ => Timeframe::Unknown,
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        if let Some(trade) = result["trade"].as_object() {
                            for key in trade.keys() {
                                match key.as_str() {
                                    "conditions" => {
                                        if let Some(conditions) = trade["conditions"].as_array() {
                                            for condition in conditions {
                                                if let Some(c) = condition.as_i64() {
                                                    contract.trade.conditions.push(c)
                                                }
                                            }
                                        }
                                    }
                                    "exchange_id" => {
                                        if let Some(exchange_id) = trade["exchange_id"].as_i64() {
                                            contract.trade.exchange_id = exchange_id
                                        }
                                    }
                                    "price" => {
                                        if let Some(price) = trade["price"].as_f64() {
                                            contract.trade.price = price
                                        }
                                    }
                                    "sip_timestamp" => {
                                        if let Some(sip_timestamp) = trade["sip_timestamp"].as_i64()
                                        {
                                            contract.trade.sip_timestamp = sip_timestamp
                                        }
                                    }
                                    "size" => {
                                        if let Some(size) = trade["size"].as_i64() {
                                            contract.trade.size = size
                                        }
                                    }
                                    "timeframe" => {
                                        if let Some(timeframe) = trade["timeframe"].as_str() {
                                            contract.trade.timeframe = match timeframe {
                                                "DELAYED" => Timeframe::Delayed,
                                                "REAL-TIME" => Timeframe::RealTime,
                                                _ => Timeframe::Unknown,
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        if let Some(open_interest) = result["open_interest"].as_i64() {
                            contract.open_interest = open_interest
                        }
                        if let Some(underlying_asset) = result["underlying_asset"].as_object() {
                            for key in underlying_asset.keys() {
                                match key.as_str() {
                                    "change_to_break_even" => {
                                        if let Some(change_to_break_even) =
                                            underlying_asset["change_to_break_even"].as_f64()
                                        {
                                            contract.underlying_asset.change_to_break_even =
                                                change_to_break_even
                                        }
                                    }
                                    "last_updated" => {
                                        if let Some(last_updated) =
                                            underlying_asset["last_updated"].as_i64()
                                        {
                                            contract.underlying_asset.last_updated = last_updated
                                        }
                                    }
                                    "price" => {
                                        if let Some(price) = underlying_asset["price"].as_f64() {
                                            contract.underlying_asset.price = price
                                        }
                                    }
                                    "ticker" => {
                                        if let Some(ticker) = underlying_asset["ticker"].as_str() {
                                            contract.underlying_asset.ticker = ticker.to_string()
                                        }
                                    }
                                    "timeframe" => {
                                        if let Some(timeframe) =
                                            underlying_asset["timeframe"].as_str()
                                        {
                                            contract.underlying_asset.timeframe = match timeframe {
                                                "DELAYED" => Timeframe::Delayed,
                                                "REAL-TIME" => Timeframe::RealTime,
                                                _ => Timeframe::Unknown,
                                            }
                                        }
                                    }
                                    "value" => {
                                        if let Some(value) = underlying_asset["value"].as_f64() {
                                            contract.underlying_asset.value = value
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        self.results.push(contract);
                    }
                }
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
