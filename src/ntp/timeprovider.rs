use fixed::types::extra::U16;
use fixed::types::extra::U32;
use fixed::FixedU32;
use fixed::FixedU64;
use rustntp::systime::SystemTime;
use rustntp::systime::SECONDS_TO_UNIX;

pub trait TimeProvider {
    fn new() -> Self;
    fn get_leap_indicator(&self) -> u8;
    fn get_stratum(&self) -> u8;
    fn get_precision(&self) -> i8;
    fn get_root_delay(&self) -> FixedU32<U16>;
    fn get_root_dispersion(&self) -> FixedU32<U16>;
    fn get_ref_id(&self) -> [u8; 4];
    fn get_ntp64_timestamp(&self) -> FixedU64<U32>;
}

pub struct LoopbackProvider;

impl TimeProvider for LoopbackProvider {
    fn new() -> Self {
        Self {}
    }

    fn get_leap_indicator(&self) -> u8 {
        3
    }

    fn get_stratum(&self) -> u8 {
        1
    }

    fn get_precision(&self) -> i8 {
        if let Ok(timespec) = SystemTime::clock_precision() {
            let nanoseconds = timespec.tv_nsec();
            let precision = 10_f32.powi(-9) * nanoseconds as f32;
            let precision_exponent = precision.to_string().split(".").last().unwrap().len() as i8;
            return -1 * precision_exponent;
        } else {
            return 9;
        }
    }

    fn get_root_delay(&self) -> FixedU32<U16> {
        FixedU32::<U16>::from_num(1.5)
    }

    fn get_root_dispersion(&self) -> FixedU32<U16> {
        FixedU32::<U16>::from_num(1.5)
    }

    fn get_ref_id(&self) -> [u8; 4] {
        // LOCL
        [76, 79, 67, 76]
    }

    fn get_ntp64_timestamp(&self) -> FixedU64<U32> {
        let default_timestamp = FixedU64::<U32>::from_num(0.0);
        SystemTime::get_ntp_epoch().unwrap_or(default_timestamp)
    }
}

pub struct TimeProviderCache {
    pub leap_indicator: u8,
    pub startum: u8,
    pub precision: i8,
    pub root_delay: FixedU32<U16>,
    pub root_dispersion: FixedU32<U16>,
    pub ref_id: [u8; 4],
    pub last_sync_timestamp: FixedU64<U32>,
}
