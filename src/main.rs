use log::info;
use std::env;
use thruster::hyper_server::HyperServer;
use thruster::ThrusterServer;
use tokio;

mod app;
pub(crate) mod controllers;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "4321".to_string());
    info!("Starting server at {}:{}", host, port);

    let app = app::init().await;
    let server = HyperServer::new(app);
    server.build(&host, port.parse::<u16>().unwrap()).await;
}
