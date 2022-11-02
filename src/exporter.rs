use axum::{extract::Extension, http::header, response::IntoResponse, routing::get, Router};
use prometheus_client::{encoding::text::encode, registry::Registry};
use std::sync::Arc;
use tokio::task;

pub(crate) fn get_router(registry: Registry) -> Router {
    Router::new()
        .route("/metrics", get(export))
        .layer(Extension(Arc::new(registry)))
}

async fn export(
    Extension(registry): Extension<Arc<Registry>>,
) -> axum::response::Result<impl IntoResponse> {
    let buf = task::spawn_blocking(move || -> std::io::Result<Vec<_>> {
        let mut buf = Vec::new();
        encode(&mut buf, &registry)?;

        Ok(buf)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    Ok((
        [(
            header::CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )],
        buf,
    ))
}
