use std::path::PathBuf;
use falco_client::config::Auth;
use falco_client::client::Client;

#[tokio::main]
async fn main() {
    let ca_path = PathBuf::from("/home/gian/stuff/ca.crt");
    let cert_path = PathBuf::from("/home/gian/stuff/client.crt");
    let key_path = PathBuf::from("/home/gian/stuff/client.key");

    let auth = Auth::new(ca_path, cert_path, key_path);

    let client = Client::new("http://[::1]:5060", auth.unwrap()).await;

    println!("{:?}", client.unwrap());
}
