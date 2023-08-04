use neo4rs::{query, Graph, Node};

use crate::{users::User, LandmarksError};

pub async fn list_users(graph: &Graph) -> Result<Vec<User>, LandmarksError> {
    let user_match = "MATCH (user:User)";
    let full_query = format!("{user_match} RETURN user");

    let mut result = graph.execute(query(&full_query)).await?;
    let mut users: Vec<User> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let user_node: Node = row
            .get("user")
            .ok_or(LandmarksError::GraphDeserializationError {
                message: "no_user_node".to_string(),
            })?;
        let name: String =
            user_node
                .get("name")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_user_name".to_string(),
                })?;

        users.push(User { name });
    }

    Ok(users)
}
