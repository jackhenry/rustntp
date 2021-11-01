use crate::protocol::ntske::*;

#[derive(Debug)]
pub struct ClientEstablishment {
    pub end_of_message: Option<EndOfMessageRecord>,
    pub next_protocol_negotiation: Option<NextProtocolNegotiationRecord>,
    pub aead_algorithm_negotiation: Option<ClientAEADAlgorithmRecord>,
    pub server_negotiation: Option<ServerNegotiationRecord>,
    pub port_negotiation: Option<PortNegotiationRecord>,
}

impl ClientEstablishment {
    pub fn new() -> Self {
        Self {
            end_of_message: None,
            next_protocol_negotiation: None,
            aead_algorithm_negotiation: None,
            server_negotiation: None,
            port_negotiation: None,
        }
    }
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
