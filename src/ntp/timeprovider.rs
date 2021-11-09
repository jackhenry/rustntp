use fixed::types::extra::U16;
use fixed::types::extra::U32;
use fixed::FixedU32;
use fixed::FixedU64;
use rustntp::systime::SystemTime;

pub trait TimeProvider {
    fn new() -> Self;
    fn get_leap_indicator(&self) -> u8;
    fn get_stratum(&self) -> u8;
    fn get_precision(&self) -> i8;
    fn get_root_delay(&self) -> FixedU32<U16>;
    fn get_root_dispersion(&self) -> FixedU32<U16>;
    fn get_ref_id(&self) -> String;
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
            let seconds = timespec.tv_sec();
            return -1 * (seconds.to_string().len() as i8 - 2);
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

    fn get_ref_id(&self) -> String {
        String::from("LOCL")
    }

    fn get_ntp64_timestamp(&self) -> FixedU64<U32> {
        let time_spec_result = SystemTime::get_ntp_epoch();
        if let Err(_error) = time_spec_result {
            return FixedU64::<U32>::from_num(0.0);
        }
        let time_spec = time_spec_result.unwrap();
        let fraction = time_spec.tv_nsec() as f64 * 10_f64.powi(-9);
        return FixedU64::<U32>::from_num(time_spec.tv_sec() as f64 + fraction);
    }
}
