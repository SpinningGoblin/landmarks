use axum::http::HeaderMap;

use crate::config::app_state::AppState;

pub fn check_auth(headers: &HeaderMap, app_state: &AppState) -> Option<String> {
    if let Some((user, pass)) = super::handlers::headers::extract_auth(headers) {
        if app_state.check_auth(&user, &pass) {
            Some(user)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn check_admin(headers: &HeaderMap, app_state: &AppState) -> bool {
    if let Some(token) = super::handlers::headers::extract_admin_token(headers) {
        app_state.check_admin(&token)
    } else {
        false
    }
}
