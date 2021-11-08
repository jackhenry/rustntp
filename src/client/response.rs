use std::collections::HashMap;

#[derive(Debug)]
pub enum ExecutionResponseType {
    NTPSyncResponse(NTPSync),
}

#[derive(Debug)]
pub struct NTPSync {}

impl NTPSync {}
