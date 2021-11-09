// IANA NTS Next Protocol Registry
// Only NTP is supported current
pub const NTS_NEXT_PROTOCOL_NTP_ID: u16 = 0;

pub mod ntp {
    use std::fmt::Debug;

    use chrono::DateTime;
    use chrono::NaiveDateTime;
    use chrono::Utc;

    use crate::helper;

    const SECONDS_TO_UNIX: i64 = 2208988800;

    pub struct Timestamp {
        pub seconds: u64,
        pub fraction: f64,
    }

    impl Timestamp {
        pub fn from(buffer_slice: &[u8], precision: i8) -> Self {
            println!("buffer: {:?}", buffer_slice);
            let seconds = helper::combine_bytes_to_u32_be(
                &buffer_slice[0],
                &buffer_slice[1],
                &buffer_slice[2],
                &buffer_slice[3],
            );
            let fraction_field = helper::combine_bytes_to_u32_be(
                &buffer_slice[4],
                &buffer_slice[5],
                &buffer_slice[6],
                &buffer_slice[7],
            );

            if seconds == 0 && fraction_field == 0 {
                return Self {
                    seconds: 0,
                    fraction: 0.0,
                };
            }

            let mut fraction = fraction_field as f64 / u32::MAX as f64;
            fraction = (fraction * 10_f64.powi(precision.abs() as i32)).trunc()
                / 10_f64.powi(precision.abs() as i32);
            Self {
                seconds: seconds as u64,
                fraction,
            }
        }

        pub fn to_date_str(&self) -> String {
            let unix: i64 = self.seconds as i64 - SECONDS_TO_UNIX;
            let naive = NaiveDateTime::from_timestamp(unix, 0);
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
            return format!(
                "{}.{}",
                newdate,
                self.fraction.to_string().split(".").last().unwrap()
            );
        }
    }

    impl Debug for Timestamp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Timestamp")
                .field("seconds", &self.seconds)
                .field("fraction", &self.fraction)
                .field("date string", &self.to_date_str())
                .finish()
        }
    }
}

pub mod ntske {
    pub const DEFAULT_NTS_PORT: u16 = 4406;
    // Recrod type numbers defined in RFC 8915
    pub const END_OF_MESSAGE_RECORD_TYPE_NUMBER: u16 = 0;
    pub const NTS_NEXT_PROTOCOL_RECORD_TYPE_NUMBER: u16 = 1;
    pub const ERROR_MESSAGE_RECORD_TYPE_NUMBER: u16 = 2;
    pub const WARNING_MESSAGE_RECORD_TYPE_NUMBER: u16 = 3;
    pub const AEAD_ALGORITHM_RECORD_TYPE_NUMBER: u16 = 4;
    pub const NEW_COOKIE_NTPV4_RECORD_TYPE_NUMBER: u16 = 5;
    pub const NTPV4_SERVER_NEGOTIATION_RECORD_TYPE: u16 = 6;
    pub const NTPV4_PORT_NEGOTIATION_RECORD_TYPE: u16 = 7;

    #[derive(Debug)]
    pub struct EndOfMessageRecord {}

    #[derive(Debug)]
    pub struct NextProtocolNegotiationRecord {
        pub protocol_ids: Vec<u16>,
    }

    #[derive(Debug)]
    pub struct ErrorRecord {
        pub error_code: u16,
    }

    #[derive(Debug)]
    pub struct WarningRecord {
        pub warning_code: u16,
    }

    /// Server AEAD negotiation can only return one 16 bit uint
    #[derive(Debug)]
    pub struct ServerAEADAlgorithmRecord {
        pub algorithm: u16,
    }

    /// Client AEAD negotiation can return an array of 16 bit uint
    #[derive(Debug)]
    pub struct ClientAEADAlgorithmRecord {
        pub algorithms: Vec<u16>,
    }

    #[derive(Debug)]
    pub struct Cookie {
        pub index: u16,
        pub nonce: u16,
        pub cipher: u32,
    }

    #[derive(Debug)]
    pub struct NewCookieRecord {
        pub cookies: Vec<Cookie>,
    }

    #[derive(Debug)]
    pub struct ServerNegotiationRecord {
        pub server_address: String,
    }

    #[derive(Debug)]
    pub struct PortNegotiationRecord {
        pub port: u16,
    }
}
