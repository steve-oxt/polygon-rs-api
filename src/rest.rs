pub mod error;
pub mod market;
pub mod parameters;
pub mod reference;

use crate::ErrorCode;
use crate::RegexPatterns;
use crate::{Parameter, ParameterRequirment, Parameters};
use chrono::DateTime;
use parameters::{TickerType, TickerTypes};
use serde_json::Value;
use std::string::ToString;

//#[derive(serde::Deserialize)]
pub enum Rest {
    Market(market::Market),
}

pub trait Request {
    const BASE_URL: &'static str = "https://api.polygon.io";
    const VERSION: &'static str;
    const CALL: &'static str;
    const PARAMETERS: &'static [&'static ParameterRequirment];

    fn parameters(&self) -> &Parameters;

    fn url(&mut self) -> &String;

    fn set_url(&mut self) -> Result<(), ErrorCode>;

    fn date_error(&self, date_type: &Parameter) -> ErrorCode {
        match date_type {
            Parameter::From => ErrorCode::DateFromError,
            Parameter::To => ErrorCode::DateToError,
            Parameter::Date => ErrorCode::DateError,
            _ => ErrorCode::WrongParameterType,
        }
    }

    fn verify_to_from(&self) -> Result<(), ErrorCode> {
        if self.parameters().to.is_none() || self.parameters().from.is_none() {
            return Ok(());
        }
        let to_string = match &self.parameters().to {
            Some(t) => t,
            None => return Err(ErrorCode::ToNotSet),
        };
        let from_string = match &self.parameters().from {
            Some(f) => f,
            None => return Err(ErrorCode::FromNotSet),
        };
        let from = match DateTime::parse_from_str(from_string.as_str(), "%Y-%m-%dT%H:%M:%S") {
            Ok(d) => d,
            Err(_) => match from_string.parse::<i64>() {
                Ok(n) => DateTime::from_timestamp_nanos(n).fixed_offset(),
                Err(_) => return Err(ErrorCode::DateFromError),
            },
        };
        let to = match DateTime::parse_from_str(to_string.as_str(), "%Y-%m-%dT%H:%M:%S") {
            Ok(d) => d,
            Err(_) => match to_string.parse::<i64>() {
                Ok(n) => DateTime::from_timestamp_nanos(n).fixed_offset(),
                Err(_) => return Err(ErrorCode::DateToError),
            },
        };
        if to < from {
            return Err(ErrorCode::DateToError);
        }

        Ok(())
    }

    fn verify_to_from_strike_price(&self) -> Result<(), ErrorCode> {
        if self.parameters().to.is_none() || self.parameters().from.is_none() {
            return Ok(());
        }
        let to_strike_price = match &self.parameters().strike_price_to {
            Some(t) => t,
            None => return Err(ErrorCode::ToNotSet),
        };
        let from_strike_price = match &self.parameters().strike_price_from {
            Some(f) => f,
            None => return Err(ErrorCode::FromNotSet),
        };
        if to_strike_price < from_strike_price {
            return Err(ErrorCode::StrikePriceToError);
        }

        Ok(())
    }

    fn verify_to_from_ticker(&self) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn verify_api_key(&self) -> Result<(), ErrorCode> {
        if !RegexPatterns::api_key().is_match(&self.parameters().api_key.as_str()) {
            return Err(ErrorCode::APIError);
        }
        Ok(())
    }

    fn verify_timestamp<T: ToString>(
        &self,
        date_value: &T,
        date_type: &Parameter,
    ) -> Result<(), ErrorCode> {
        let date = date_value.to_string();
        match RegexPatterns::epoch_nano_date().is_match(date.as_str()) {
            true => Ok(()),
            false => Err(self.date_error(date_type)),
        }
    }

    fn verify_date<T: ToString>(
        &self,
        date_value: &T,
        date_type: &Parameter,
    ) -> Result<(), ErrorCode> {
        let date = date_value.to_string();
        if date.parse::<i64>().is_ok() {
            match self.verify_timestamp(date_value, date_type) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        };
        match RegexPatterns::string_date().is_match(date.as_str()) {
            true => Ok(()),
            false => Err(self.date_error(date_type)),
        }
    }

    fn verify_stock_ticker(&self, ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::stocks_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_stocks_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => self.verify_stock_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_option_ticker(&self, ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::options_ticker().is_match(&ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_options_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => self.verify_option_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_indicie_ticker(&self, ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::indicies_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_indices_ticker(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => self.verify_indicie_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_forex_ticker(&self, ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::forex_ticker().is_match(&ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_forex_tickers(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => self.verify_forex_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_crypto_ticker(&self, ticker: String) -> Result<(), ErrorCode> {
        if !RegexPatterns::forex_ticker().is_match(ticker.as_str()) {
            return Err(ErrorCode::TickerError);
        }
        Ok(())
    }

    fn verify_crypto_tickers(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => self.verify_crypto_ticker(t.to_string()),
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn get_ticker_type(&self, ticker: &String) -> Result<TickerType, ErrorCode> {
        if RegexPatterns::stocks_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Stocks);
        }
        if RegexPatterns::options_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Options);
        }
        if RegexPatterns::indicies_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Indicies);
        }
        if RegexPatterns::forex_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Forex);
        }
        if RegexPatterns::indicies_check().is_match(ticker.as_str()) == true {
            return Ok(TickerType::Crypto);
        }
        return Err(ErrorCode::TickerError);
    }

    fn verify_ticker(&self, required: bool, ticker_types: &TickerTypes) -> Result<(), ErrorCode> {
        match &self.parameters().ticker {
            Some(t) => match self.get_ticker_type(t) {
                Ok(ticker_type) => match ticker_type {
                    TickerType::Stocks => {
                        if !ticker_types.stocks {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        self.verify_stocks_ticker(required)
                    }
                    TickerType::Options => {
                        if !ticker_types.options {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        self.verify_options_ticker(required)
                    }
                    TickerType::Indicies => {
                        if !ticker_types.indicies {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        self.verify_indices_ticker(required)
                    }
                    TickerType::Forex => {
                        if !ticker_types.forex {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        self.verify_forex_tickers(required)
                    }
                    TickerType::Crypto => {
                        if !ticker_types.crypto {
                            return Err(ErrorCode::TickerNotValidForAPICall);
                        }
                        self.verify_crypto_tickers(required)
                    }
                },
                Err(e) => return Err(e),
            },
            None => {
                if required {
                    return Err(ErrorCode::TickerNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_tickers(&self, required: bool, ticker_types: &TickerTypes) -> Result<(), ErrorCode> {
        match &self.parameters().tickers {
            Some(tickers) => {
                for ticker in tickers {
                    match self.get_ticker_type(ticker) {
                        Ok(ticker_type) => match ticker_type {
                            //Better Error Message
                            TickerType::Stocks => {
                                if !ticker_types.stocks {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                self.verify_stock_ticker(ticker.to_string())?
                            }
                            TickerType::Options => {
                                if !ticker_types.options {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                self.verify_option_ticker(ticker.to_string())?
                            }
                            TickerType::Indicies => {
                                if !ticker_types.indicies {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                self.verify_indicie_ticker(ticker.to_string())?
                            }
                            TickerType::Forex => {
                                if !ticker_types.forex {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                self.verify_forex_ticker(ticker.to_string())?
                            }
                            TickerType::Crypto => {
                                if !ticker_types.crypto {
                                    return Err(ErrorCode::TickerNotValidForAPICall);
                                };
                                self.verify_crypto_ticker(ticker.to_string())?
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            None => {
                if required {
                    return Err(ErrorCode::TickersNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify_underlying_asset(&self, required: bool) -> Result<(), ErrorCode> {
        match &self.parameters().underlying_asset {
            Some(underlying_asset) => {
                match self.get_ticker_type(underlying_asset) {
                    Ok(ticker_type) => match ticker_type {
                        //Better Error Message
                        TickerType::Stocks => {
                            self.verify_stock_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Options => {
                            self.verify_option_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Indicies => {
                            self.verify_indicie_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Forex => {
                            self.verify_forex_ticker(underlying_asset.to_string())?
                        }
                        TickerType::Crypto => {
                            self.verify_crypto_ticker(underlying_asset.to_string())?
                        }
                    },
                    Err(e) => return Err(e),
                }
                Ok(())
            }
            None => {
                if required {
                    return Err(ErrorCode::UnderlyingAssetNotSet);
                };
                Ok(())
            }
        }
    }

    fn verify<T: ToString>(
        &self,
        required: bool,
        parameter_value: &Option<T>,
        parameter_type: &Parameter,
    ) -> Result<(), ErrorCode> {
        match parameter_value {
            Some(p) => match parameter_type {
                Parameter::Date => self.verify_date(p, parameter_type),
                Parameter::To => self.verify_date(p, parameter_type),
                Parameter::From => self.verify_date(p, parameter_type),
                Parameter::Timestamp => self.verify_timestamp(p, parameter_type),
                _ => Ok(()),
            },
            None => {
                if required {
                    match parameter_type {
                        Parameter::Adjusted => return Err(ErrorCode::AdjusteedNotSet),
                        Parameter::Sort => return Err(ErrorCode::SortNotSet),
                        Parameter::Limit => return Err(ErrorCode::LimitNotSet),
                        Parameter::Timespan => return Err(ErrorCode::TimespanNotSet),
                        Parameter::Multiplier => return Err(ErrorCode::MultiplierNotSet),
                        Parameter::IncludeOTC => return Err(ErrorCode::IncludeOTCNotSet),
                        Parameter::Order => return Err(ErrorCode::OrderNotSet),
                        Parameter::Sortv3 => return Err(ErrorCode::SortNotSet),
                        Parameter::Timestamp => return Err(ErrorCode::TimestampNotSet),
                        Parameter::ContractType => return Err(ErrorCode::ContractTypeNotSet),
                        Parameter::StrikePrice => return Err(ErrorCode::StrikePriceNotSet),
                        _ => return Err(ErrorCode::WrongParameterType),
                    }
                };
                Ok(())
            }
        }
    }

    fn check_parameters(&self, ticker_types: &TickerTypes) -> Result<(), ErrorCode> {
        if let Err(check) = self.verify_api_key() {
            return Err(check);
        }
        for parameter in Self::PARAMETERS {
            match parameter.parameter {
                Parameter::Ticker => {
                    if let Err(check) = self.verify_ticker(parameter.required, ticker_types) {
                        return Err(check);
                    }
                }
                Parameter::Tickers => {
                    if let Err(check) = self.verify_tickers(parameter.required, ticker_types) {
                        return Err(check);
                    }
                }
                Parameter::UnderlyingAsset => {
                    if let Err(check) = self.verify_underlying_asset(parameter.required) {
                        return Err(check);
                    }
                }
                Parameter::TickerFrom => {}
                Parameter::TickerTo => {}
                Parameter::Date => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().date,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Adjusted => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().adjusted,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Sort => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().sort,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Limit => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().adjusted,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Timespan => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().timespan,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::From => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().from,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::To => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().to,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Multiplier => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().multiplier,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::IncludeOTC => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().include_otc,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Order => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().order,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Sortv3 => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().sortv3,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Timestamp => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().timestamp,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::ContractType => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().contract_type,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePrice => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().strike_price,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePriceFrom => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().strike_price_from,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::StrikePriceTo => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().strike_price_to,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Amount => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().amount,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Precision => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().precision,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
                Parameter::Direction => {
                    if let Err(check) = self.verify(
                        parameter.required,
                        &self.parameters().direction,
                        &parameter.parameter,
                    ) {
                        return Err(check);
                    }
                }
            }
        }
        if let Err(check) = self.verify_to_from() {
            return Err(check);
        }
        if let Err(check) = self.verify_to_from_strike_price() {
            return Err(check);
        }
        if let Err(check) = self.verify_to_from_ticker() {
            return Err(check);
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

    fn request(&mut self) -> Result<(), ErrorCode>;
}
