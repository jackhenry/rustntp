use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use fixed::types::extra::U32;
use fixed::FixedU64;
use rustntp::systime::SystemTime;
use tokio::net::UdpSocket;
use tokio::runtime;

use crate::handler::ClientModeHandler;
use crate::sync::ProviderSynchronizer;
use crate::timeprovider::TimeProvider;
use crate::timeprovider::TimeProviderCache;

pub struct Server<T>
where
    T: TimeProvider,
{
    pub socket: UdpSocket,
    pub provider: T,
    pub provider_cache: Arc<Mutex<TimeProviderCache>>,
}

impl<T> Server<T>
where
    T: TimeProvider,
{
    pub fn new(socket: UdpSocket, provider: T) -> Self {
        // sync system clock on initialization and update cache values
        let mut last_sync_timestamp = provider.get_ntp64_timestamp();
        if let Err(error) = SystemTime::set_time_from_ntp_epoch(last_sync_timestamp) {
            tracing::error!("Unable to set system clock.");
            tracing::error!("Nix error: {}", error);
            last_sync_timestamp = FixedU64::<U32>::from_num(0.0);
        }

        let provider_cache = Arc::new(Mutex::new(TimeProviderCache {
            leap_indicator: provider.get_leap_indicator(),
            startum: provider.get_stratum(),
            precision: provider.get_precision(),
            root_delay: provider.get_root_delay(),
            root_dispersion: provider.get_root_dispersion(),
            ref_id: provider.get_ref_id(),
            last_sync_timestamp,
        }));

        let new_server = Self {
            socket,
            provider,
            provider_cache,
        };

        new_server
    }

    pub async fn run(self) -> Result<(), rustntp::Error> {
        let mut to_send: Option<(usize, SocketAddr)> = None;
        let mut buffer = [0; 1024];

        /* std::thread::spawn(|| {
            let runtime_result = runtime::Builder::new_current_thread().enable_all().build();

            if let Err(runtime_error) = runtime_result {
                tracing::error!("Unable to spawn synchronization thread.")
                panic!("{}", runtime_error);

            }

            let runtime = runtime_result.unwrap();

            runtime.block_on(async {
                let provider_synchronizer =
                    ProviderSynchronizer::new(&self.provider, &self.provider_cache, 5000);
                provider_synchronizer.run_loop().await;
            })
        }); */

        loop {
            // Wait for next packet to receive
            tracing::debug!("waiting for next packet");
            to_send = match self.socket.recv_from(&mut buffer).await {
                Ok(message) => Some(message),
                Err(error) => {
                    tracing::debug!("Read error {:?}", error);
                    None
                }
            };

            if let Some((size, peer)) = to_send {
                // NTP packet should be at least 48 bytes
                if size < 48 {
                    tracing::debug!(
                        "Received a packet that didn't reach minimum length of 48 bytes."
                    );
                    continue;
                }
                let message = &buffer[..size];
                let client_handler =
                    ClientModeHandler::new(&self.socket, &peer, &message, &self.provider_cache);
                client_handler.process().await;
            };
        }
    }
}
