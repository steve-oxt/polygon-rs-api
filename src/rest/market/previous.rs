use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Previous {
    previous_parameters: Parameters,
    previous_url: String,
    pub adjusted: bool,
    pub query_count: i64,
    pub request_id: String,
    pub results: Vec<Bar>,
    pub results_count: i64,
    pub status: String,
    pub ticker: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct Bar {
    pub ticker: String,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub timestamp: i64,
    pub volume: f64,
    pub volume_weighted: f64,
}

impl Previous {
    pub fn set_parameters(&mut self, api_key: String, ticker: String, adjusted: Option<bool>) {
        self.previous_parameters = Parameters {
            api_key: api_key,
            ticker: Some(ticker),
            adjusted: adjusted,
            ..Parameters::default()
        }
    }
}

impl Request for Previous {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Ticker,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.previous_parameters
    }

    fn url(&mut self) -> &String {
        &self.previous_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::all()) {
            return Err(check);
        }
        self.previous_url = String::from(format!(
            "{}/{}/{}/ticker/{}/prev?{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().ticker.unwrap(),
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
                if let Some(adjusted) = response["adjusted"].as_bool() {
                    self.adjusted = adjusted
                }
                if let Some(request_id) = response["request_id"].as_str() {
                    self.request_id = request_id.to_string()
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(ticker) = response["ticker"].as_str() {
                    self.ticker = ticker.to_string()
                }
                if let Some(query_count) = response["queryCount"].as_i64() {
                    self.query_count = query_count
                }
                if let Some(results_count) = response["resultsCount"].as_i64() {
                    self.results_count = results_count
                }
                if let Some(results) = response["results"].as_array() {
                    for result in results {
                        let mut bar = Bar::default();
                        if let Some(ticker) = result["T"].as_str() {
                            bar.ticker = ticker.to_string()
                        }
                        if let Some(close) = result["c"].as_f64() {
                            bar.close = close
                        }
                        if let Some(high) = result["h"].as_f64() {
                            bar.high = high
                        }
                        if let Some(low) = result["l"].as_f64() {
                            bar.low = low
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
                        self.results.push(bar);
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
