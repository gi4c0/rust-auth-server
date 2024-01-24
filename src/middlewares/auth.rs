use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::utils::jwt;

pub async fn auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = headers.get(AUTHORIZATION).ok_or(StatusCode::UNAUTHORIZED)?;
    let token = token.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_data = jwt::verify(token).map_err(|e| {
        dbg!(e);
        StatusCode::UNAUTHORIZED
    })?;

    request.extensions_mut().insert(user_data);

    Ok(next.run(request).await)
}
