use nix::sys;
use nix::sys::time::TimeSpec;
use nix::time;

pub const SECONDS_TO_UNIX: i64 = 2208988800;
pub struct SystemTime;

impl SystemTime {
    pub fn clock_precision() -> nix::Result<sys::time::TimeSpec> {
        time::clock_getres(time::ClockId::CLOCK_REALTIME)
    }

    pub fn get_ntp_epoch() -> Result<TimeSpec, crate::Error> {
        let time_spec_result = time::clock_gettime(time::ClockId::CLOCK_REALTIME);
        if let Err(_time_spec) = time_spec_result {
            return Err(crate::Error::SystemTimeError(String::from(
                "Unable to retrieve system time",
            )));
        };

        let time_spec = time_spec_result.unwrap();
        let seconds = time_spec.tv_sec() + SECONDS_TO_UNIX;
        if seconds < 0 {
            return Err(crate::Error::SystemTimeError(String::from(
                "System NTP time is negative.",
            )));
        }

        Ok(time_spec)
    }
}
