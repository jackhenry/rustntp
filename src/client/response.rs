use std::collections::HashMap;

#[derive(Debug)]
pub enum ExecutionResponseType {
    KeyEstablishmentResponse(NTSServers),
    ClockSync,
}

#[derive(Debug)]
pub struct NTSServers {
    pub error_map: HashMap<String, rustntp::Error>,
    pub cookie_map: HashMap<String, Vec<u8>>,
}

impl NTSServers {
    pub fn new() -> Self {
        Self {
            error_map: HashMap::new(),
            cookie_map: HashMap::new(),
        }
    }
}
