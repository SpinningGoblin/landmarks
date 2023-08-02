use neo4rs::Graph;

#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub username: String,
    pub uri: String,
    pub password: String,
}

impl ConnectionConfig {
    pub fn load_env() -> Result<Self, anyhow::Error> {
        let uri = std::env::var("NEO4J_CONNECTION_URI")?;
        let password = std::env::var("NEO4J_PASSWORD")?;

        Ok(Self {
            username: "neo4j".to_string(),
            uri,
            password,
        })
    }

    pub async fn to_graph(&self) -> Result<Graph, anyhow::Error> {
        let graph = Graph::new(&self.uri, &self.username, &self.password).await?;
        Ok(graph)
    }
}
