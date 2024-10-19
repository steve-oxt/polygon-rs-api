use crate::{
    ErrorCode, Parameter, ParameterRequirment, Parameters, Request, Sort, TickerTypes, Timespan,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Aggregates {
    aggregates_parameters: Parameters,
    aggregates_url: String,
    pub adjusted: bool,
    pub next_url: String,
    pub request_id: String,
    pub results: Vec<Bar>,
    pub status: String,
    pub results_count: i64,
    pub ticker: String,
    pub query_count: i64,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Bar {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub transactions: i64,
    pub open: f64,
    pub timestamp: i64,
    pub volume: f64,
    pub volume_weighted: f64,
    pub otc: bool,
}

impl Aggregates {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        ticker: String,
        multiplier: u16,
        timespan: Timespan,
        from: String,
        to: String,
        sort: Option<Sort>,
        limit: Option<u16>,
        adjusted: Option<bool>,
    ) {
        self.aggregates_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            adjusted: adjusted,
            multiplier: Some(multiplier),
            timespan: Some(timespan),
            from: Some(from),
            to: Some(to),
            sort: sort,
            limit: limit,
            ..Parameters::default()
        }
    }
}

impl Request for Aggregates {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Multiplier,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Timespan,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::From,
        },
        &ParameterRequirment {
            required: true,
            parameter: Parameter::To,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Sort,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Limit,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.aggregates_parameters
    }

    fn url(&mut self) -> &String {
        &self.aggregates_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::all()) {
            return Err(check);
        }
        if self.next_url != "" {
            self.aggregates_url = format!(
                "{}&apiKey={}",
                self.next_url.to_string(),
                self.parameters().clone().api_key
            );
            return Ok(());
        }
        self.aggregates_url = String::from(format!(
            "{}/{}/{}/ticker/{}/range/{}/{}/{}/{}?{}{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
            self.parameters().clone().multiplier.unwrap(),
            self.parameters().clone().timespan.unwrap(),
            self.parameters().clone().from.unwrap(),
            self.parameters().clone().to.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().sort {
                format!("sort={}&", s)
            } else {
                "".to_string()
            },
            if let Some(l) = self.parameters().clone().limit {
                format!("limit={}&", l)
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
                if let Some(adjusted) = response["adjusted"].as_bool() {
                    self.adjusted = adjusted
                }
                if let Some(next_url) = response["next_url"].as_str() {
                    self.next_url = next_url.to_string()
                } else {
                    self.next_url = "".to_string()
                }
                if let Some(query_count) = response["queryCount"].as_i64() {
                    self.query_count = query_count
                }
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(results_count) = response["resultsCount"].as_i64() {
                    self.results_count = results_count
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(ticker) = response["ticker"].as_str() {
                    self.ticker = ticker.to_string()
                }
                if let Some(results) = response["results"].as_array() {
                    for result in results {
                        let mut bar = Bar::default();
                        if let Some(close) = result["c"].as_f64() {
                            bar.close = close
                        }
                        if let Some(high) = result["h"].as_f64() {
                            bar.high = high
                        }
                        if let Some(low) = result["l"].as_f64() {
                            bar.low = low
                        }
                        if let Some(transactions) = result["n"].as_i64() {
                            bar.transactions = transactions
                        }
                        if let Some(open) = result["o"].as_f64() {
                            bar.open = open
                        }
                        if let Some(timestamp) = result["t"].as_i64() {
                            bar.timestamp = timestamp
                        }
                        if let Some(volume) = result["v"].as_f64() {
                            bar.volume = volume
                        }
                        if let Some(volume_weighted) = result["vw"].as_f64() {
                            bar.volume_weighted = volume_weighted
                        }
                        if let Some(otc) = result["otc"].as_bool() {
                            bar.otc = otc
                        }
                        self.results.push(bar);
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
