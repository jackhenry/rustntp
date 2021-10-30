PORT="${1:-9001}" 
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

certificate=( $parent_path/../cert.pem )
private=( $parent_path/../private.pem )
RUST_LOG=tokio=trace cargo run --bin ntske "127.0.0.1:$PORT" -c $certificate -k $private 