use neo4rs::Graph;

use crate::LandmarksError;

#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub username: String,
    pub uri: String,
    pub password: String,
}

impl ConnectionConfig {
    pub fn load_env() -> Result<Self, LandmarksError> {
        let uri = std::env::var("NEO4J_CONNECTION_URI").map_err(|_| {
            LandmarksError::MissingEnvVariable {
                name: "NEO4J_CONNECTION_URI".to_string(),
            }
        })?;
        let password =
            std::env::var("NEO4J_PASSWORD").map_err(|_| LandmarksError::MissingEnvVariable {
                name: "NEO4J_PASSWORD".to_string(),
            })?;

        Ok(Self {
            username: "neo4j".to_string(),
            uri,
            password,
        })
    }

    pub async fn to_graph(&self) -> Result<Graph, LandmarksError> {
        let graph = Graph::new(&self.uri, &self.username, &self.password).await?;
        Ok(graph)
    }
}
