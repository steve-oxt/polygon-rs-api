use std::str;

use crate::data_types::Parse;
use crate::rest::parameters::{ContractStyle, ContractType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Details {
    pub contract_type: Option<ContractType>,
    pub contract_style: Option<ContractStyle>,
    pub expiration_date: Option<String>,
    pub shares_per_contract: Option<i64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
}

impl Parse for Details {
    fn parse(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let contract_type = match Self::string_parse(map, vec!["contract_type"]) {
            Some(contract_type) => match contract_type.as_str() {
                "Call" => Some(ContractType::Call),
                "Put" => Some(ContractType::Put),
                _ => None,
            },
            None => None,
        };
        let contract_style = match Self::string_parse(map, vec!["contract_style"]) {
            Some(contract_style) => match contract_style.as_str() {
                "American" => Some(ContractStyle::American),
                "European" => Some(ContractStyle::European),
                "Bermudan" => Some(ContractStyle::Bermudan),
                _ => None,
            },
            None => None,
        };
        let expiration_date = Self::string_parse(map, vec!["expiration_date"]);
        let shares_per_contract = Self::i64_parse(map, vec!["shares_per_contract"]);
        let strike_price = Self::f64_parse(map, vec!["strike_price"]);
        let ticker = Self::string_parse(map, vec!["ticker"]);
        Details {
            contract_type,
            contract_style,
            expiration_date,
            shares_per_contract,
            strike_price,
            ticker,
        }
    }
}

#[test]
fn test_details_parse() {
    let data = serde_json::json!({
        "contract_type": "Call",
        "contract_style": "American",
        "expiration_date": "2023-03-03",
        "shares_per_contract": 100,
        "strike_price": 10.0,
        "ticker": "TEST"
    });
    let details = Details::parse(&data.as_object().unwrap());
    assert_eq!(details.contract_type.unwrap(), ContractType::Call);
    assert_eq!(details.contract_style.unwrap(), ContractStyle::American);
    assert_eq!(details.expiration_date.unwrap(), "2023-03-03");
    assert_eq!(details.shares_per_contract.unwrap(), 100);
    assert_eq!(details.strike_price.unwrap(), 10.0);
    assert_eq!(details.ticker.unwrap(), "TEST");
}
