//! Application builder and startup.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::routes;
use crate::state::AppState;

/// Build the Axum router with all routes and middleware.
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(routes::search_routes())
        .merge(routes::api_routes())
        .merge(routes::static_routes())
        .merge(routes::health_routes())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Start the server.
pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    let addr = SocketAddr::from((
        state.settings.server.host.parse::<std::net::IpAddr>()?,
        state.settings.server.port,
    ));

    let app = build_router(state);

    info!("Metasearch server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
