pub mod daily;
pub mod nbbo;
pub mod trades;
pub mod snapshots;
pub mod aggregates;
pub mod grouped;
pub mod last_quote;
pub mod last_trade;
pub mod previous;
pub mod technical_indicators;
pub mod rtc;
pub mod currency_quote;
pub mod bbo;
pub mod ltc;

#[derive(serde::Deserialize)]
pub enum Market{
    Daily(daily::Daily),
}