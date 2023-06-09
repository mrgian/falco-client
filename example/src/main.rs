use std::path::PathBuf;
use falco_client::config::Auth;
use falco_client::client::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let ca_path = PathBuf::from("certificates/ca.crt");
    let cert_path = PathBuf::from("certificates/client.crt");
    let key_path = PathBuf::from("certificates/client.key");

    let auth = Auth::new(ca_path, cert_path, key_path);

    let mut client = Client::new("http://[::1]:5060", auth?).await?;
    let messages = client.get();

    for message in messages.await? {
        println!("{}", message.output);
    }

    Ok(())
}
