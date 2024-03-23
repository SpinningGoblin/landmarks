use neo4rs::{query, Graph};
use serde::Deserialize;

use crate::{users::User, LandmarksError};

#[derive(Deserialize)]
struct UserNode {
    pub name: String,
}

#[derive(Deserialize)]
struct UserRow {
    pub user: UserNode,
}

pub async fn list_users(graph: &Graph) -> Result<Vec<User>, LandmarksError> {
    let user_match = "MATCH (user:User)";
    let full_query = format!("{user_match} RETURN user");

    let mut result = graph.execute(query(&full_query)).await?;
    let mut users: Vec<User> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let user_row =
            row.to::<UserRow>()
                .map_err(|e| LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                })?;

        users.push(User {
            name: user_row.user.name,
        });
    }

    Ok(users)
}
