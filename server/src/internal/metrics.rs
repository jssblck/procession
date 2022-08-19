use std::{collections::HashMap, sync::Arc, time::Duration};

use strum::{Display, EnumCount, EnumIter};
use tokio::sync::RwLock;

use crate::boundedvec;

use super::bounded_vec::BoundedVec;

/// The measurements that are collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter, Display)]
pub enum Measure {
    #[strum(serialize = "redis_latency")]
    RedisLatency,
    #[strum(serialize = "endpoint_latency")]
    EndpointLatency,
}

#[derive(Debug)]
struct Inner {
    latencies: HashMap<Measure, BoundedVec<Duration>>,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            latencies: HashMap::with_capacity(Measure::COUNT),
        }
    }
}

/// Records metrics for API operations.
/// Thread safe; can be freely cloned but still refers to the same underlying data.
#[derive(Debug, Clone)]
pub struct Collection {
    bound: usize,
    inner: Arc<RwLock<Inner>>,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            bound: 1000,
            inner: Default::default(),
        }
    }
}

impl Collection {
    /// Create a new `Metrics` instance with the provided historical bounds.
    pub fn new(bound: usize) -> Self {
        Self {
            bound,
            inner: Arc::new(RwLock::new(Inner::default())),
        }
    }

    /// Wrap a function to hydrate latency.
    pub async fn rec_latency(&self, field: Measure, latency: Duration) {
        self.inner
            .write()
            .await
            .latencies
            .entry(field)
            .and_modify(|v| v.push(latency))
            .or_insert_with(|| boundedvec![self.bound; latency]);
    }
}
