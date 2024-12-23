pub mod exponential_moving_average;
pub mod moving_average_converge_divergence;
pub mod relative_strength_index;
pub mod simple_moving_average;

use exponential_moving_average::ExponentialMovingAverageRequest;
use moving_average_converge_divergence::MovingAverageConvergenceDivergenceRequest;
use relative_strength_index::RelativeStrengthIndexRequest;
use serde::{Deserialize, Serialize};
use simple_moving_average::SimpleMovingAverageRequest;

#[derive(Serialize, Deserialize)]
pub enum TechnicalIndicators {
    ExponentialMovingAverage(exponential_moving_average::ExponentialMovingAverage),
    MovingAverageConvergenceDivergence(
        moving_average_converge_divergence::MovingAverageConvergenceDivergence,
    ),
    RelativeStrengthIndex(relative_strength_index::RelativeStrengthIndex),
    SimpleMovingAverage(simple_moving_average::SimpleMovingAverage),
}

pub struct TechnicalIndicatorsRequest {}

impl ExponentialMovingAverageRequest for TechnicalIndicatorsRequest {}

impl MovingAverageConvergenceDivergenceRequest for TechnicalIndicatorsRequest {}

impl RelativeStrengthIndexRequest for TechnicalIndicatorsRequest {}

impl SimpleMovingAverageRequest for TechnicalIndicatorsRequest {}
