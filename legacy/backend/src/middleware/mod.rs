use actix_web::http::header::HeaderMap;

pub mod auth;
pub mod project;

pub fn get_token_from_headers(headers: &HeaderMap) -> String {
    headers.get("Authorization") 
        .and_then(|h| h.to_str().ok())
        .and_then(|h| {
            h.strip_prefix("Bearer ")
                .map(|stripped_token | stripped_token.to_owned())
        })
        .unwrap_or_else(String::new)
}