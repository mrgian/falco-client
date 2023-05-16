use falco::{service_client::ServiceClient, Request};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

pub mod config;
pub mod client;

pub mod falco {
    tonic::include_proto!("falco.outputs");
}
