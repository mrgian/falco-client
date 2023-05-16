use std::path::PathBuf;
use falco_client::config::Auth;

fn main() {
    let ca_path = PathBuf::from("/home/gian/stuff/ca.crt");
    let cert_path = PathBuf::from("/home/gian/stuff/client.crt");
    let key_path = PathBuf::from("/home/gian/stuff/client.key");

    let auth = Auth::new(ca_path, cert_path, key_path);

    println!("{:?}", auth.unwrap());
}
