pub mod gainers_losers;
pub mod indicies_snapshot;
pub mod l2_snapshot;
pub mod options_chain;
pub mod options_contract;
pub mod ticker_snapshot;
pub mod tickers_snapshot;
pub mod universal_snapshot;

use gainers_losers::GainersLosersRequest;
use indicies_snapshot::IndiciesSnapshotRequest;
use l2_snapshot::L2SnapshotRequest;
use options_chain::OptionsChainRequest;
use options_contract::OptionsContractRequest;
use serde::{Deserialize, Serialize};
use ticker_snapshot::TickerSnapshotRequest;
use tickers_snapshot::TickersSnapshotRequest;
use universal_snapshot::UniversalSnapshotRequest;

#[derive(Serialize, Deserialize)]
pub enum Snapshot {
    GainersLosers(gainers_losers::GainersLosers),
    IndicesSnapshot(indicies_snapshot::IndiciesSnapshot), //Done but need to compleate to and from ticker verifications
    L2Snapshot(l2_snapshot::L2Snapshot),
    OptionsChain(options_chain::OptionsChain),
    OptionsContract(options_contract::OptionsContract),
    TickerSnapshot(ticker_snapshot::TickerSnapshot),
    TickersSnapshot(tickers_snapshot::TickersSnapshot),
    UniversalSnapshot(universal_snapshot::UniversalSnapshot), //Done but need to compleate to and from ticker verifications
}

pub trait SnapshotRequest {}

impl GainersLosersRequest for dyn SnapshotRequest {}

impl IndiciesSnapshotRequest for dyn SnapshotRequest {}

impl L2SnapshotRequest for dyn SnapshotRequest {}

impl OptionsChainRequest for dyn SnapshotRequest {}

impl OptionsContractRequest for dyn SnapshotRequest {}

impl TickerSnapshotRequest for dyn SnapshotRequest {}

impl TickersSnapshotRequest for dyn SnapshotRequest {}

impl UniversalSnapshotRequest for dyn SnapshotRequest {}