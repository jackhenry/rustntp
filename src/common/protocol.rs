// IANA NTS Next Protocol Registry
// Only NTP is supported current
pub const NTS_NEXT_PROTOCOL_NTP_ID: u16 = 0;

pub mod ntp {
    use std::fmt::Debug;

    use chrono::DateTime;
    use chrono::NaiveDateTime;
    use chrono::Utc;

    use crate::Helper;

    const SECONDS_TO_UNIX: u32 = 2208988800;

    pub struct Timestamp {
        pub value: f64,
    }

    impl Timestamp {
        pub fn from(buffer_slice: &[u8], precision: i8) -> Self {
            let seconds = Helper::combine_bytes_to_u32_be(
                &buffer_slice[0],
                &buffer_slice[1],
                &buffer_slice[2],
                &buffer_slice[3],
            );
            let fraction = Helper::combine_bytes_to_u32_be(
                &buffer_slice[4],
                &buffer_slice[5],
                &buffer_slice[6],
                &buffer_slice[7],
            );

            if seconds == 0 && fraction == 0 {
                return Self { value: 0.0 };
            }

            let mut decimal_fraction = Helper::fraction_bits_to_decimal(precision, fraction);
            // Let n equal the leading digits of the decimal fraction.
            // multiple the fraction by 10^-n in order to move all significant bits to right of decimal place
            let leading_digits = decimal_fraction.round().to_string().len();
            decimal_fraction = decimal_fraction * (10_f32.powf(-1_f32 * leading_digits as f32));
            // Final epoch is the seconds added to the fraction
            let final_epoch = (seconds as f64) + (decimal_fraction as f64);

            Self { value: final_epoch }
        }

        pub fn to_date_str(&self) -> String {
            let unix = self.value - (SECONDS_TO_UNIX as f64);
            let naive = NaiveDateTime::from_timestamp(
                unix.round() as i64,
                (unix.fract() * 10_f64.powf(9_f64)) as u32,
            );
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S%.f");
            return format!("{}", newdate);
        }
    }

    impl Debug for Timestamp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Timestamp")
                .field("value", &self.value)
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
