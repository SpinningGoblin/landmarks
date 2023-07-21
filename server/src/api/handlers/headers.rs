use axum::http::HeaderMap;

pub const USER_HEADER: &str = "Landmark-User";
pub const ADMIN_HEADER: &str = "Landmark-Admin-Token";

pub fn extract_user(headers: &HeaderMap) -> Option<String> {
    if let Some(header_value) = headers.get(USER_HEADER) {
        header_value.to_str().map(|value| value.to_string()).ok()
    } else {
        None
    }
}

pub fn extract_auth_token(headers: &HeaderMap) -> Option<String> {
    if let Some(header_value) = headers.get("Authorization") {
        header_value
            .to_str()
            .map(|value| value.replace("Bearer ", ""))
            .ok()
    } else {
        None
    }
}

pub fn extract_auth(headers: &HeaderMap) -> Option<(String, String)> {
    extract_user(headers).zip(extract_auth_token(headers))
}

pub fn extract_admin_token(headers: &HeaderMap) -> Option<String> {
    if let Some(header_value) = headers.get(ADMIN_HEADER) {
        header_value.to_str().map(|value| value.to_string()).ok()
    } else {
        None
    }
}
