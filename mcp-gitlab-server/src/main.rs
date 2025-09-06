use tokio;
use tracing::{info, Level};
use tracing_subscriber;

use rmcp::transport::streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager};

// Include the gitlab module
mod gitlab;

const BIND_ADDRESS: &str = "127.0.0.1:7000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let service = StreamableHttpService::new(
        || Ok(gitlab::GitLabTool::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    info!("Starting GitLab MCP Server.");

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;
    Ok(())
}
