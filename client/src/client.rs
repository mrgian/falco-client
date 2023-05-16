use crate::config::Auth;
use crate::falco::{service_client::ServiceClient, Request, Response};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use std::error::Error;

#[derive(Debug)]
pub struct Client {
    pub service: ServiceClient<Channel>,
}

impl Client {
    pub async fn new(endpoint: &'static str, auth: Auth) -> Result<Client, Box<dyn Error>>{
        let identity = Identity::from_pem(auth.cert, auth.key);

        let tls = ClientTlsConfig::new()
            .ca_certificate(auth.ca)
            .identity(identity)
            .domain_name("localhost");

        let channel = Channel::from_static(endpoint)
            .tls_config(tls)?
            .connect()
            .await?;

        let service_client = ServiceClient::new(channel);

        let service = Self {
            service: service_client
        };

        Ok(service)
    }

    pub async fn get(&mut self) -> Result<Response, Box<dyn Error>>{
        let request = tonic::Request::new(Request {});
        let response = self.service.get(request).await?;
        let message = response.into_inner().message().await?.unwrap();

        Ok(message)
    }
}