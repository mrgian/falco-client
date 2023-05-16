#!/bin/bash

set -e 

#cleanup
rm -rf ./certificates
mkdir ./certificates

#generate CA root certificate
openssl genrsa -out ./certificates/ca.key 2048
openssl req -new -x509 -days 365 -key ./certificates/ca.key -subj "/C=CN/ST=GD/L=SZ/O=localhost/CN=localhost" -out ./certificates/ca.crt

#generate server certificate
openssl req -newkey rsa:2048 -nodes -keyout ./certificates/server.key -subj "/C=CN/ST=GD/L=SZ/O=localhost/CN=localhost" -out ./certificates/server.csr
openssl x509 -req -extfile <(printf "subjectAltName=DNS:localhost") -days 365 -in ./certificates/server.csr -CA ./certificates/ca.crt -CAkey ./certificates/ca.key -CAcreateserial -out ./certificates/server.crt

#generate client certificate
openssl req -newkey rsa:2048 -nodes -keyout ./certificates/client.key -subj "/C=CN/ST=GD/L=SZ/O=localhost/CN=localhost" -out ./certificates/client.csr
openssl x509 -req -extfile <(printf "subjectAltName=DNS:localhost") -days 365 -in ./certificates/client.csr -CA ./certificates/ca.crt -CAkey ./certificates/ca.key -CAcreateserial -out ./certificates/client.crt