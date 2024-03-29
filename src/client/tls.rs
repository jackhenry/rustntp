use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use rustls::OwnedTrustAnchor;
use rustls::RootCertStore;

pub struct TLS {}

impl TLS {
    pub fn client_connection_for(server_address: &String) -> rustls::ClientConnection {
        let mut root_cert_store = RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            println!("{:?}", cert.0);
            root_cert_store.add(&rustls::Certificate(cert.0)).unwrap();
        }

        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        let server_name = server_address.as_str().try_into().unwrap();
        println!("dns name: {:?}", server_name);

        rustls::ClientConnection::new(Arc::new(config), server_name).unwrap()
    }
}
