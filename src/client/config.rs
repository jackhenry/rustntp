use std::default::Default;

#[derive(Serialize, Deserialize)]
struct ClientConfig {
    version: u8,
    api_key: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            version: 0,
            api_key: "".into(),
        }
    }
}
