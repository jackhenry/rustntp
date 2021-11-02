use crate::protocol::ntske::*;

#[derive(Debug)]
pub struct ClientEstablishment {
    pub end_of_message: EndOfMessageRecord,
    pub next_protocol_negotiation: NextProtocolNegotiationRecord,
    pub aead_algorithm_negotiation: ClientAEADAlgorithmRecord,
    pub server_negotiation: ServerNegotiationRecord,
    pub port_negotiation: PortNegotiationRecord,
}

impl ClientEstablishment {
    
}

pub struct ServerEstablishment {
    pub end_of_message: EndOfMessageRecord,
    pub next_protocol_negotiation: NextProtocolNegotiationRecord,
    pub error_notification: ErrorRecord,
    pub warning_notification: WarningRecord,
    pub aead_algorithm_negotiation: ServerAEADAlgorithmRecord,
    pub new_cookie: NewCookieRecord,
    pub server_negotiation: ServerNegotiationRecord,
    pub port_negotiation: PortNegotiationRecord,
}

impl ServerEstablishment {}
