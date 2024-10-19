pub mod aggregates;
pub mod bbo;
pub mod currency_conversion;
pub mod daily;
pub mod grouped;
pub mod last_quote;
pub mod last_trade;
pub mod pair_quote;
pub mod pair_trade;
pub mod previous;
pub mod quotes;
pub mod snapshots;
pub mod technical_indicators;
pub mod trades;

#[derive(serde::Deserialize)]
pub enum Market {
    Aggregates(aggregates::Aggregates), //Done
    Grouped(grouped::Grouped),          //Done
    Daily(daily::Daily),                //Done
    Previous(previous::Previous),       //Done
    Trades(trades::Trades),             //Done
    LastTrade(last_trade::LastTrade),   //Done
    Quotes(quotes::Quotes),             //Done
    LastQuote(last_quote::LastQuote),   //Done
    Snapshots(snapshots::Snapshots),
    TechnicalIndicators(technical_indicators::TechnicalIndicators),
    CurrencyConversion(currency_conversion::CurrencyConversion), //Done
    PairQuote(pair_quote::PairQuote),                            //Done
    BBO(bbo::BBO),                                               //Done
    PairTrade(pair_trade::PairTrade),                            //Done
}
