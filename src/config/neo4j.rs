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
}
