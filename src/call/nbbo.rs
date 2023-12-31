use crate::Polygon;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBBO {
    pub next_url: Option<String>,
    pub request_id: Option<String>,
    pub results: Option<Vec<Quote>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub ask_exchange: Option<i32>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i32>,
    pub bid_exchange: Option<i32>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<i32>,
    pub indicators: Option<Vec<i32>>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub tape: Option<i32>,
}

impl NBBO {
    #[tokio::main]
    pub async fn nbbo(p: Polygon) -> Result<NBBO, Box<dyn Error>> {
        let api_key = match p.api_key {
            Some(a) => a,
            None => panic!("There is no api key set"),
        };
        let mut url_options = String::from("");
        match p.ticker {
            Some(t) => {
                url_options = format!("{}?", t);
            }
            None => panic!("There is no ticker set"),
        };
        match p.date {
            Some(d) => {
                url_options = format!("{}timestamp={}&", url_options, d);
            }
            None => {
                if p.verbose == Some(true) {
                    println!("There is no date set, trying from and to.");
                }
                match p.from {
                    Some(f) => {
                        url_options = format!("{}timestamp.gte={}&", url_options, f);
                    }
                    None => { if p.verbose == Some(true) { println!("There is no from set")} },
                };
                match p.to {
                    Some(t) => {
                        url_options = format!("{}timestamp.lt={}&", url_options, t);
                    }
                    None => { if p.verbose == Some(true) { println!("There is no to set")} },
                };
            }
        };

        match p.sort {
            Some(s) => {
                url_options = format!("{}order={:?}&sort=timestamp&", url_options, s);
            }
            None => { if p.verbose == Some(true) { println!("There is no sort set")} },
        };
        match p.limit {
            Some(l) => {
                url_options = format!("{}limit={}&", url_options, l);
            }
            None => { if p.verbose == Some(true) { println!("There is no limit set")} },
        };

        let url = format!(
            "https://api.polygon.io/v3/quotes/{}apiKey={}",
            url_options, api_key
        );
        let request = match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => panic!("The following error occured: {}", e),
            },
            Err(e) => panic!("The following error occured: {}", e),
        };
        match serde_json::from_str(request.as_str()) {
            Ok(nbbo) => Ok(nbbo),
            Err(e) => panic!("The following error occured: {}", e),
        }
    }
}
