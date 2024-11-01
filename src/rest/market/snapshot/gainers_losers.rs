use crate::data_types::{ticker::Ticker, Parse};
use crate::rest::{
    error::ErrorCode,
    parameters::{Direction, Parameter, ParameterRequirment, Parameters, TickerType, TickerTypes},
};
use crate::tools::{request::Request, verification::Verification};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GainersLosers {
    pub status: Option<String>,
    pub tickers: Option<Vec<Ticker>>,
}

impl GainersLosersRequest for GainersLosers {}

impl Parse for GainersLosers {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let status = Self::string_parse(map, vec!["status"]);
        let tickers = Self::array_parse(map, vec!["tickers"]);
        GainersLosers { status, tickers }
    }
}

pub trait GainersLosersRequest {
    fn get_gainers_losers(
        &self,
        api_key: String,
        direction: Direction,
        include_otc: Option<bool>,
        ticker_type: TickerType,
    ) -> Result<GainersLosers, ErrorCode> {
        let ticker_types = match ticker_type {
            TickerType::Stocks => TickerTypes::stocks(),
            TickerType::Forex => TickerTypes::forex(),
            TickerType::Crypto => TickerTypes::crypto(),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let includeotc = match ticker_type {
            TickerType::Forex | TickerType::Crypto => None,
            _ => include_otc,
        };
        let gainers_losers_parameters = Parameters {
            api_key: api_key,
            direction: Some(direction),
            include_otc: includeotc,
            ..Parameters::default()
        };
        if let Err(check) =
            Verification::check_parameters(&ticker_types, PARAMETERS, &gainers_losers_parameters)
        {
            return Err(check);
        }
        let locale = match ticker_type {
            TickerType::Stocks => String::from("us"),
            TickerType::Forex | TickerType::Crypto => String::from("global"),
            _ => return Err(ErrorCode::TickerTypeeNotValidForAPICall),
        };
        let url = match url(&gainers_losers_parameters, locale, ticker_type){
            Ok(url) => url,
            Err(e) => return Err(e)
        };
        match Request::request(url) {
            Ok(mut map) => Ok(GainersLosers::parse(&mut map)),
            Err(e) => return Err(e),
        }
    }
}

const PARAMETERS: &'static [&'static ParameterRequirment] = &[
    &ParameterRequirment {
        required: true,
        parameter: Parameter::Direction,
    },
    &ParameterRequirment {
        required: false,
        parameter: Parameter::IncludeOTC,
    },
];

fn url(parameters: &Parameters, locale: String, ticker_type: TickerType) -> Result<String, ErrorCode> {
    let url =String::from(format!(
        "https://api.polygon.io/v2/snapshot/locale/{}/markets/{}/{}?{}apiKey={}",
        locale,
        ticker_type.to_string().to_lowercase(),
        if let Some(s) = &parameters.direction {
            format!("direction={}&", s.to_string().to_lowercase())
        } else {
            "".to_string()
        },
        if let Some(s) = &parameters.include_otc {
            format!("include_otc={}&", s)
        } else {
            "".to_string()
        },
        &parameters.api_key,
    ));
    Ok(url)
}
