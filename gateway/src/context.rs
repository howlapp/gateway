//! Defines the shared context between REST and GraphQL services.

use std::sync::Arc;

use furink_proto::discovery::discovery_service_client::DiscoveryServiceClient;
use tokio::sync::{mpsc, RwLock};
use tonic::transport::Channel;
use warp::ws::Message;

/// A client connected to the gateway.
pub struct Client {
    pub has_identified: bool,
    pub connection: (mpsc::UnboundedSender<Message>),
}

/// The root-level context. All references to this context must be
/// immutable, and individual children should be wrapped in the `RwLock`
/// type.
#[derive(Debug)]
pub struct Context {
    /// The service discovery client used for fetching available services.
    pub discovery_client: RwLock<DiscoveryServiceClient<Channel>>,
}

/// Thread-save immutable context.
pub type ThreadContext = Arc<Context>;
