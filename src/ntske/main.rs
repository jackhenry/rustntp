use argh::FromArgs;
use rustls::server::NoClientAuth;
use rustls_pemfile::{certs, rsa_private_keys};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{copy, sink, split, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{self, Runtime};
use tokio::sync::Mutex;
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use tracing::metadata::LevelFilter;

use crate::client_handler::ClientHandler;

mod client_handler;
mod decoder;
mod keygen;

/// Tokio Rustls server example
#[derive(FromArgs)]
struct Options {
    /// bind addr
    #[argh(positional)]
    addr: String,

    /// cert file
    #[argh(option, short = 'c')]
    cert: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,
}

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::FULL)
        .init();

    let options: Options = argh::from_env();
    let addr = options
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
    let certs = load_certs(&options.cert)?;
    let mut keys = load_keys(&options.key)?;

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(NoClientAuth::new())
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await?;
    tracing::debug!("server running on {}", addr);

    loop {
        let (stream, peer_address) = listener.accept().await?;
        let acceptor = acceptor.clone();
        tracing::trace!("Accepted new incoming connection from {}", peer_address);
        let handler_result = ClientHandler::run(stream, acceptor, peer_address)
            .await
            .unwrap();

        /* match handler_result {
            Ok(a) => {
                tracing::trace!("{:?}", a);
            }
            Err(error) => {
                tracing::error!("from {}: {:?}", peer_address, error);
            }
        }; */
    }
    Ok(())
}

type KeyMaterial = Vec<u8>;

async fn process(
    stream: TcpStream,
    acceptor: TlsAcceptor,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut stream = acceptor.accept(stream).await?;
    let (_, connection) = stream.get_ref();
    // Export key material
    let mut output: Vec<u8> = vec![0; 128];
    let label: Vec<u8> = vec![0; 40];
    // Key material is copied to the output vector
    if let Err(err) = connection.export_keying_material(&mut output, &label, None) {
        tracing::debug!("{:?}", err);
    }

    stream.write("Your key material:\n".as_bytes()).await?;
    let hex = format!("{:X?}", &output).to_string();
    stream.write(hex.as_bytes()).await?;

    Ok(())
}
