parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

private_path=$parent_path/../private.pem
public_path=$parent_path/../public.pem
cert_path=$parent_path/../cert.pem

openssl genrsa -out $private_path 2048

openssl rsa -in $private_path -pubout -out $public_path

openssl req -new -x509 -key $private_path -out $cert_path -days 365