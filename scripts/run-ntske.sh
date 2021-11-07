PORT="${1:-9001}" 
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

certificate=( $parent_path/../keys/end.cert)
private=( $parent_path/../keys/end.key )
RUST_LOG=ntske=trace cargo run --bin ntske "127.0.0.1:$PORT" -c $certificate -k $private 