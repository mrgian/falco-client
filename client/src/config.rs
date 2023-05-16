use std::path::PathBuf;
use std::error::Error;
use tonic::transport::Certificate;

//certificates for tls authentication
#[derive(Debug)]
pub struct Auth {
    pub ca: Certificate, //root ca certificate
    pub cert: Certificate, //client certificate
    pub key: Certificate, //client private key
}

impl Auth {
    //creates new auth using certificates from the given paths
    pub fn new(ca_path: PathBuf, cert_path: PathBuf, key_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let ca_string = std::fs::read_to_string(ca_path)?;
        let cert_string = std::fs::read_to_string(cert_path)?;
        let key_string = std::fs::read_to_string(key_path)?;

        let auth = Self {
            ca: Certificate::from_pem(ca_string),
            cert: Certificate::from_pem(cert_string),
            key: Certificate::from_pem(key_string),
        };

        Ok(auth)
    }
}