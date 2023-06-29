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
