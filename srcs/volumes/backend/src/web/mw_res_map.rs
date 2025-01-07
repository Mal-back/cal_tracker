use std::sync::Arc;

use crate::web;
use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::{ctx::Ctx, log::log_request};

pub async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("{:<12} - main response mapper", "RESPONSE MAPPER");

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Arc<web::Error>>();
    debug!(
        "{:<12} - main response mapper service error : {:?}",
        "RESPONSE MAPPER", service_error
    );
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    debug!(
        "{:<12} - main response mapper client error : {:?}",
        "RESPONSE MAPPER", client_status_error
    );

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            debug!("client error body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    debug!("\n");
    error_response.unwrap_or(res)
}
