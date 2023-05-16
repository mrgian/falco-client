# falco-client
gRPC API client for [Falco](https://github.com/falcosecurity/falco)

![falco-client](https://github.com/mrgian/falco-client/assets/10211171/8292a3af-d31c-4b55-a677-ab3ea64a3c32)

## Usage
### Clone the repo
`git clone https://github.com/mrgian/falco-client`

### Generate the certificates
`cd falco-client`<br>
`./gen_crt.sh`

### Configure falco
Edit `/etc/falco/falco.yaml`, uncomment these lines and adjust paths to the dir where you cloned the repo

```zsh
grpc:
  enabled: true
  bind_address: "0.0.0.0:5060"
  threadiness: 8
  private_key: "/home/gian/falco-client/certificates/certificates/server.key"
  cert_chain: "/home/gian/falco-client/certificates/certificates/server.crt"
  root_certs: "/home/gian/falco-client/certificates/certificates/ca.crt"
```
Don't forget to enable gRPC output too:
```zsh
grpc_output:
  enabled: true
```

### Running the example
Simulate a suspicious event:
`sudo cat /etc/shadow > /dev/null`

And then run the example:
`cargo run --bin example`

You should see the outputs of the events.
