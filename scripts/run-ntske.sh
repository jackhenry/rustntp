PORT="${1:-9001}" 
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

certificate=( $parent_path/../keys/localhost-cert.pem)
private=( $parent_path/../keys/localhost-key.pem )
RUST_LOG=ntske=trace cargo run --bin ntske "localhost:$PORT" -c $certificate -k $private 