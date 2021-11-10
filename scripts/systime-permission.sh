parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

sudo setcap cap_sys_time+eip $parent_path/../target/debug/ntp