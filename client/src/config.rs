use std::path::PathBuf;
use std::error::Error;
use tonic::transport::Certificate;

#[derive(Debug)]
pub struct Auth {
    pub ca: Certificate,
    pub cert: Certificate,
    pub key: Certificate,
}

impl Auth {
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