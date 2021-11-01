parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

HOST=$1
PORT=$2

cargo run --bin client $HOST $PORT