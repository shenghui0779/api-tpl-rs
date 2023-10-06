use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{config, result::response::ApiErr, util::auth::Identity};

pub async fn handle<B>(
    State(state): State<config::AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    match request.extensions().get::<Identity>() {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(identity) => match identity.check(&state.db).await {
            Err(err) => return ApiErr::ErrAuth(Some(err.to_string())).into_response(),
            Ok(_) => next.run(request).await,
        },
    }
}
