use crate::{
    rest::parameters::TickerTypes, ErrorCode, Parameter, ParameterRequirment, Parameters, Request,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Grouped {
    grouped_parameters: Parameters,
    grouped_url: String,
    pub adjusted: bool,
    pub results: Vec<Bar>,
    pub status: String,
    pub results_count: i64,
    pub query_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Bar {
    pub excahnge: String,
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

impl Grouped {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        date: String,
        include_otc: Option<bool>,
        adjusted: Option<bool>,
    ) {
        self.grouped_parameters = Parameters {
            api_key: api_key,
            date: Some(date),
            adjusted: adjusted,
            include_otc: include_otc,
            ..Parameters::default()
        }
    }
}

impl Request for Grouped {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs/grouped";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::IncludeOTC,
        },
    ];

    fn parameters(&self) -> &Parameters {
        &self.grouped_parameters
    }

    fn url(&mut self) -> &String {
        &self.grouped_url
    }

    fn set_url(&mut self) -> Result<(), ErrorCode> {
        if let Err(check) = self.check_parameters(&TickerTypes::set(true, false, false, true, true))
        {
            return Err(check);
        }
        self.grouped_url = String::from(format!(
            "{}/{}/{}/locale/us/market/stocks/{}?{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().date.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
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
                if let Some(adjusted) = response["adjusted"].as_bool() {
                    self.adjusted = adjusted
                }
                if let Some(query_count) = response["queryCount"].as_i64() {
                    self.query_count = query_count
                }
                if let Some(results_count) = response["resultsCount"].as_i64() {
                    self.results_count = results_count
                }
                if let Some(status) = response["status"].as_str() {
                    self.status = status.to_string()
                }
                if let Some(results) = response["results"].as_array() {
                    for result in results {
                        let mut bar = Bar::default();
                        if let Some(exchange) = result["T"].as_str() {
                            bar.excahnge = exchange.to_string()
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
