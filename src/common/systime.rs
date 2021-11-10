use std::time::Duration;

use fixed::types::extra::U32;
use fixed::types::extra::U64;
use fixed::FixedU64;
use nix::sys;
use nix::sys::time::TimeSpec;
use nix::time;

pub const SECONDS_TO_UNIX: i64 = 2208988800;
pub struct SystemTime;

impl SystemTime {
    pub fn clock_precision() -> nix::Result<sys::time::TimeSpec> {
        time::clock_getres(time::ClockId::CLOCK_REALTIME)
    }

    pub fn get_ntp_epoch() -> Result<FixedU64<U32>, crate::Error> {
        let time_spec_result = time::clock_gettime(time::ClockId::CLOCK_REALTIME);
        if let Err(_time_spec) = time_spec_result {
            return Err(crate::Error::SystemTimeError(String::from(
                "Unable to retrieve system time",
            )));
        };

        if let Err(_error) = time_spec_result {
            return Ok(FixedU64::<U32>::from_num(0.0));
        }
        let time_spec = time_spec_result.unwrap();
        let seconds = (time_spec.tv_sec() + SECONDS_TO_UNIX) as f64;
        let fraction = time_spec.tv_nsec() as f64 * 10_f64.powi(-9);
        if seconds < 0.0 {
            return Err(crate::Error::SystemTimeError(String::from(
                "System NTP time is negative.",
            )));
        }

        return Ok(FixedU64::<U32>::from_num(seconds + fraction));
    }

    pub fn set_time_from_ntp_epoch(timestamp: FixedU64<U32>) -> nix::Result<()> {
        let seconds = timestamp.int().to_num::<f64>() - SECONDS_TO_UNIX as f64;
        let fraction = timestamp.frac().to_num::<f64>();
        let duration = Duration::from_secs_f64(seconds + fraction);
        let timespec = nix::sys::time::TimeSpec::from_duration(duration);
        nix::time::clock_settime(nix::time::ClockId::CLOCK_REALTIME, timespec)
    }
}

#[cfg(test)]
mod test {
    use super::SystemTime;

    #[test]
    fn test_clock_precision() {
        let time_spec_result = SystemTime::clock_precision();
        assert!(time_spec_result.is_ok());
    }
}
