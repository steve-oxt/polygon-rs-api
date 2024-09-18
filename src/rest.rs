pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

use std::collections::HashMap;

use crate::ErrorCode;
use crate::{Parameter, ParameterRequirment, Parameters};
use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;

#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}

pub trait Request {
    const BASE_URL: &'static str = "https://api.polygon.io";
    const VERSION: &'static str;
    const CALL: &'static str;
    const PARAMETERS: &'static [&'static ParameterRequirment];
    
    fn hashma() {//-> HashMap<&'static Parameter, Box<dyn for<'a> Fn(&'a Self, bool) -> Result<(), ErrorCode>>> {
        let mut hm: HashMap<&Parameter, Box<dyn for<'a> Fn(&'a _, bool) -> Result<(), ErrorCode>>> = HashMap::new();
        hm.insert(&Parameter::Ticker, Box::new(Self::verify_ticker));
            //let mut m: HashMap<&'static Parameter, Box<dyn for<'a> Fn(&'a Self, bool) -> Result<(), ErrorCode>>> = HashMap::new();
            //m.insert(&Parameter::Ticker, Box::new(Self::verify_ticker));
        hm
        
    }

    fn hashmap() -> &'static HashMap<&Parameter, Box<dyn for<'a> Fn(&'a &Self, bool) -> Result<(), ErrorCode>>> {
        static HASHMAP: OnceLock<HashMap<&Parameter, Box<dyn for<'a> Fn(self, bool) -> Result<(), ErrorCode>>>> = OnceLock::new();
        
        HASHMAP.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert(&Parameter::Ticker, Box::new(Self::verify_ticker));
            m
        })
    }

    fn parameters(&self) -> &Parameters;

    fn url(&mut self) -> &String;

    fn set_url(&mut self) -> Result<(), ErrorCode>;

    fn set_regex(&self, pattern: &str) -> Result<Regex, ErrorCode> {
        match Regex::new(pattern) {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("The following error occured: {}", e);
                return Err(ErrorCode::RegexError);
            }
        }
    }

    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        let regex_pattern = self.set_regex(r"\S{32}");
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        if !regex_pattern
            .unwrap()
            .is_match(&self.parameters().api_key.as_str())
        {
            return Err(ErrorCode::APIError);
        }
        Ok(())
    }

    //Need to adjust Regex check for nano timestamp ^\d{19}$
    fn verify_date(&self, required: bool) -> Result<(), ErrorCode> {
        let regex_pattern =
            self.set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])");
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        match &self.parameters().date {
            Some(d) => match regex_pattern.unwrap().is_match(d.as_str()) {
                true => Ok(()),
                false => Err(ErrorCode::DateError),
            },
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    //Need to adjust Regex check for nano timestamp ^\d{19}$ and verify that the date is less or equal to the to date
    fn verify_from_date(&self, required: bool) -> Result<(), ErrorCode> {
        let regex_pattern =
            self.set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])");
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        match &self.parameters().from {
            Some(d) => match regex_pattern.unwrap().is_match(d.as_str()) {
                true => Ok(()),
                false => Err(ErrorCode::DateError),
            },
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    //Need to adjust Regex check for nano timestamp ^\d{19}$ and verify that the date is greater or equal to the from date
    fn verify_to_date(&self, required: bool) -> Result<(), ErrorCode> {
        let regex_pattern =
            self.set_regex(r"(19|20)([0-9]{2})-(1[0-2]|0[1-9])-(3[01]|[12][0-9]|0[1-9])");
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        match &self.parameters().to {
            Some(d) => match regex_pattern.unwrap().is_match(d.as_str()) {
                true => Ok(()),
                false => Err(ErrorCode::DateError),
            },
            None => {
                if required {
                    return Err(ErrorCode::DateNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_options_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        let regex_pattern = self.set_regex(
            r"(O:)([A-Z]){1,4}([0-9]{2})(1[0-2]|0[1-9])(3[01]|[12][0-9]|0[1-9])([CP]){1}([0-9]){8}",
        );
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        match &self.parameters().ticker {
            Some(t) => match regex_pattern.unwrap().is_match(t.as_str()) {
                true => Ok(()),
                false => Err(ErrorCode::OptionsTickerError),
            },
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        let regex_pattern = self.set_regex(r"^O:");
        if let Err(e) = regex_pattern {
            return Err(e);
        }
        match &self.parameters().ticker {
            Some(t) => match regex_pattern.unwrap().is_match(t.as_str()) {
                true => match self.verify_options_ticker(required) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        println!("{}", e);
                        Err(ErrorCode::TickerError)
                    }
                },
                false => Ok(()),
            },
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_adjusted(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().adjusted {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::AdjusteedNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_sort(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().sort {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::SortNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_limit(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().limit {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::LimitNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_timespan(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().timespan {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::TimespanNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_multiplier(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().multiplier {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::MultiplierNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_order(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().order {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::OrderNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_sortv3(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().sortv3 {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::SortNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_timestamp(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().timestamp {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::TimestampNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_contract_type(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().contract_type {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::MultiplierNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_include_otc(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().include_otc {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::IncludeOTCNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_strike_price(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().strike_price {
            Some(_) => Ok(()),
            None => {
                if required {
                    return Err(ErrorCode::StrikePriceNotSet);
                };
                Ok(())
            }
        }
    }

    fn check_parameters(&self) -> Result<(), ErrorCode> {
        if let Err(check) = self.verify_api_key() {
            return Err(check);
        }
        let mut hm: HashMap<&Parameter, Box<dyn for<'a> Fn(&'a _, bool) -> Result<(), ErrorCode>>> = HashMap::new();
            hm.insert(&Parameter::Ticker, Box::new(Self::verify_ticker));
        for parameter in Self::PARAMETERS {
            if let Err(check) = hm.get(&parameter.parameter).unwrap()(self, parameter.required) {
                return Err(check);
            }
            match parameter.parameter {
                Parameter::Ticker => {
                    if let Err(check) = self.verify_ticker(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Date => {
                    if let Err(check) = self.verify_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Adjusted => {
                    if let Err(check) = self.verify_adjusted(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Sort => {
                    if let Err(check) = self.verify_sort(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Limit => {
                    if let Err(check) = self.verify_limit(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Timespan => {
                    if let Err(check) = self.verify_timespan(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::From => {
                    if let Err(check) = self.verify_from_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::To => {
                    if let Err(check) = self.verify_to_date(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Multiplier => {
                    if let Err(check) = self.verify_multiplier(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::IncludeOTC => {
                    if let Err(check) = self.verify_include_otc(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::OptionsTicker => {
                    if let Err(check) = self.verify_options_ticker(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Order => {
                    if let Err(check) = self.verify_order(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Sortv3 => {
                    if let Err(check) = self.verify_sortv3(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::Timestamp => {
                    if let Err(check) = self.verify_timestamp(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::ContractType => {
                    if let Err(check) = self.verify_contract_type(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::StrikePrice => {
                    if let Err(check) = self.verify_strike_price(parameter.required) {
                        return Err(check);
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::main]
    async fn get_raw_data(&mut self) -> Result<String, ErrorCode> {
        match reqwest::get(self.url()).await {
            Ok(response) => match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => {
                    println!("{}", e);
                    return Err(ErrorCode::RequestError);
                }
            },
            Err(e) => {
                println!("{}", e);
                return Err(ErrorCode::RequestError);
            }
        }
    }

    fn return_parsed_string(&self, pattern: String, data: &String) -> String {
        let key_value_pair = Regex::new(&pattern).unwrap().find(&data).unwrap().as_str();
        let extracted_value = Regex::new(":(.*?)$")
            .unwrap()
            .find(&key_value_pair)
            .unwrap()
            .as_str();
        let striped_value = Regex::new("[\\w.]")
            .unwrap()
            .find(&extracted_value)
            .unwrap()
            .as_str();
        striped_value.to_string()
        //Regex::new(":(.*?)$").unwrap().find(&key_value_pair).unwrap().as_str().replace(":", "").replace("\"", "").replace("}","").replace(",","").to_string()
    }

    fn return_parsed_number(&self, pattern: String, data: &String) -> f64 {
        let key_value_pair = Regex::new(&pattern).unwrap().find(&data).unwrap().as_str();
        let extracted_value = Regex::new(":(.*?)$")
            .unwrap()
            .find(&key_value_pair)
            .unwrap()
            .as_str();
        let striped_value = Regex::new("[\\w.]")
            .unwrap()
            .find(&extracted_value)
            .unwrap()
            .as_str();
        striped_value.parse::<f64>().unwrap()
        //(Regex::new(":(.*?)$").unwrap().find(&key_value_pair).unwrap().as_str().replace(":", "").replace("\"", "").replace("}","").replace(",","").to_string()).parse::<f64>().unwrap()
    }

    fn return_parsed_integer(&self, pattern: String, data: &String) -> i64 {
        let key_value_pair = Regex::new(&pattern).unwrap().find(&data).unwrap().as_str();
        let extracted_value = Regex::new(":(.*?)$")
            .unwrap()
            .find(&key_value_pair)
            .unwrap()
            .as_str();
        let striped_value = Regex::new("[\\w.]")
            .unwrap()
            .find(&extracted_value)
            .unwrap()
            .as_str();
        striped_value.parse::<i64>().unwrap()
        //(Regex::new(":(.*?)$").unwrap().find(&key_value_pair).unwrap().as_str().replace(":", "").replace("\"", "").replace("}","").replace(",","").to_string()).parse::<i64>().unwrap()
    }

    fn return_parsed_array(_pattern: String, _data: &String) -> String {
        String::from("")
    }

    fn polygon_request(&mut self) -> Result<Value, ErrorCode> {
        if let Err(check) = self.set_url() {
            return Err(check);
        }
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let v: Value = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => {
                println!("{}", err);
                return Err(ErrorCode::JSONParseError);
            }
        };
        Ok(v)
    }

    fn polygon_request_string(&mut self) -> Result<String, ErrorCode> {
        if let Err(check) = self.set_url() {
            return Err(check);
        }
        match self.get_raw_data() {
            Ok(response) => Ok(response),
            Err(e) => return Err(e),
        }
    }

    fn request(&mut self) -> Result<(), ErrorCode>;
}
