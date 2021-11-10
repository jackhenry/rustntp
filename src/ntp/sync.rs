use rustntp::systime::SystemTime;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::timeprovider::TimeProvider;
use crate::timeprovider::TimeProviderCache;

pub struct ProviderSynchronizer<'a, T>
where
    T: TimeProvider,
{
    provider: &'a T,
    cache: &'a Arc<Mutex<TimeProviderCache>>,
    interval: u64,
}

impl<'a, T> ProviderSynchronizer<'a, T>
where
    T: TimeProvider,
{
    pub fn new(provider: &'a T, cache: &'a Arc<Mutex<TimeProviderCache>>, interval: u64) -> Self {
        Self {
            provider,
            cache,
            interval,
        }
    }

    pub async fn run_loop(&self) {
        loop {
            sleep(Duration::from_millis(self.interval)).await;

            let mut cache_guard = self.cache.lock().unwrap();
            cache_guard.leap_indicator = self.provider.get_leap_indicator();
            cache_guard.startum = self.provider.get_stratum();
            cache_guard.precision = self.provider.get_precision();
            cache_guard.root_delay = self.provider.get_root_delay();
            cache_guard.root_dispersion = self.provider.get_root_dispersion();
            cache_guard.ref_id = self.provider.get_ref_id();
            let current_timestamp = self.provider.get_ntp64_timestamp();
            cache_guard.last_sync_timestamp = current_timestamp;
            match SystemTime::set_time_from_ntp_epoch(current_timestamp) {
                Ok(_) => tracing::debug!("Successfully updated system time from provider."),
                Err(err) => {
                    tracing::error!("Unable to update system time.");
                    tracing::error!("Nix error: {}", err);
                }
            }
        }
    }
}
