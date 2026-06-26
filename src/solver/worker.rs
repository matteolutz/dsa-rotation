use std::sync::Arc;

use crate::weights::{PTCWeights, PTPWeights};

pub struct SolverWorkerInput<const N: usize> {
    ptp_weights: Arc<PTPWeights>,
    ptc_weights: Arc<PTCWeights<N>>,

    n_generations: usize,
}

pub struct SolverWorkerOutput {}
