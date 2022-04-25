//! Defines the shared context between REST and GraphQL services.
use furink_proto::discovery::discovery_service_client::DiscoveryServiceClient;
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use tonic::transport::Channel;
use warp::ws::Message;

/// A client connected to the gateway.
#[derive(Debug)]
pub struct Client {
    /// Whether or not this client has identified.
    pub has_identified: bool,
    /// Write-only channel that can send data to the connected client.
    pub connection: UnboundedSender<Message>,
}

/// The root-level context. All references to this context must be
/// immutable, and individual children should be wrapped in the `RwLock`
/// type.
#[derive(Debug)]
pub struct Context {
    /// The service discovery client used for fetching available services.
    pub discovery_client: RwLock<DiscoveryServiceClient<Channel>>,
    /// A map of clients connected to the gateway.
    pub clients: RwLock<Vec<RwLock<Client>>>,
}
