use std::net::SocketAddr;
use std::{env, error::Error, sync::Arc, time::Duration};

use furink_proto::version::{spawn_heartbeat_task, validate_and_register, HeartbeatConfig};

use furink_proto::discovery::{
    discovery_service_client::DiscoveryServiceClient, RegisterRequest, ServiceKind,
};
use tokio::sync::RwLock;
use tonic::transport::Endpoint;
use tracing::info;
use warp::ws::{WebSocket, Ws};
use warp::Filter;

use crate::context::Context;

mod context;

#[tracing::instrument]
async fn handle_connection(socket: WebSocket, context: Arc<Context>) {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load dotenv when in development
    if cfg!(debug_assertions) {
        dotenv::dotenv().unwrap();
    }
    println!(
        r#"
{} v{}
Authors: {}
"#,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    // setup logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // register with discovery service
    let (id, channel) = validate_and_register(
        Endpoint::from_shared(env::var("DISCOVERY_URL").expect("DISCOVERY_URL"))?,
        RegisterRequest {
            kind: ServiceKind::Gateway as i32,
            address: env::var("SERVICE_HOST").expect("SERVICE_HOST"),
            port: env::var("SERVICE_PORT").expect("SERVICE_PORT").parse()?,
        },
    )
    .await?;
    // spawn heartbeat
    spawn_heartbeat_task(HeartbeatConfig {
        channel: channel.clone(),
        id: id.clone(),
        interval: Duration::from_secs(30),
    });
    // create the context
    let context = Context {
        clients: RwLock::new(Vec::new()),
        discovery_client: RwLock::new(DiscoveryServiceClient::new(channel.clone())),
    };
    // make the context thread-safe
    let context = Arc::new(context);
    // setup context filters
    let warp_ctx = warp::any().map(move || context.clone());
    let log = warp::log("gateway");
    // create the gateway filter
    let gateway = warp::path::end()
        .and(warp::ws())
        .and(warp_ctx)
        .map(|ws: Ws, ctx| ws.on_upgrade(move |socket| handle_connection(socket, ctx)))
        .with(log);
    // parse server addr
    let url: SocketAddr = env::var("SERVICE_URL")
        .expect("SERVICE_URL")
        .parse()
        .expect("failed to parse SERVICE_URL");
	// start the server
    info!("Listening on http://127.0.0.1:8080");
    warp::serve(gateway).try_bind(url).await;
    Ok(())
}
