use landmarks_core::config::neo4j::ConnectionConfig;

use super::auth::Authentication;

#[derive(Clone, Debug)]
pub struct AppState {
    pub authentication: Authentication,
    pub connection: ConnectionConfig,
}

impl AppState {
    pub fn load_from_env() -> Self {
        Self {
            authentication: Authentication::load_from_env(),
            connection: ConnectionConfig::load_env().unwrap(),
        }
    }

    pub fn check_auth(&self, user: &str, pass: &str) -> bool {
        self.authentication.check(user, pass)
    }

    pub fn check_admin(&self, token: &str) -> bool {
        self.authentication.check_admin(token)
    }

    pub fn connection_config(&self) -> &ConnectionConfig {
        &self.connection
    }
}
