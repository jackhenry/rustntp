#[derive(Debug)]
pub enum ExecutionResponseType {
    KeyEstablishment(KeyEstablismentResponse),
    ClockSync
}


#[derive(Debug)]
pub struct KeyEstablismentResponse {
}
