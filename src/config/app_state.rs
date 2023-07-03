use neo4rs::Graph;

use super::{auth::Authentication, neo4j::ConnectionConfig};

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

    pub async fn to_graph(&self) -> Result<Graph, anyhow::Error> {
        self.connection.to_graph().await
    }
}