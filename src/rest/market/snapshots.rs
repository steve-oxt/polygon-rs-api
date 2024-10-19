pub mod chain;
pub mod contract;
pub mod gainers_losers;
pub mod indicies;
pub mod l2;
pub mod ticker;
pub mod tickers;
pub mod universal;

#[derive(serde::Deserialize)]
pub enum Snapshots {
    Chain(chain::Chain),                          //Done
    Ticker(ticker::TickerSnapshot),               //Done
    Tickers(tickers::TickersSnapshot),            //Done
    Universal,
    GainersLosers(gainers_losers::GainersLosers), //Done
    Contract(contract::Contract),                 //Done
    Indices(indicies::Indicies),                  //Done but need to compleate from verifications
    L2(l2::L2),                                   //Done
}
