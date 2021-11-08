PORT="${1:-9002}" 
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

RUST_LOG=ntp=trace cargo run --bin ntp "127.0.0.1:$PORT" 