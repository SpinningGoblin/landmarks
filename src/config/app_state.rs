use aws_sdk_s3::Client;
use neo4rs::Graph;
use uuid::Uuid;

use super::{
    auth::Authentication, backup_plan::BackupPlan, blob_storage::AwsS3Storage,
    neo4j::ConnectionConfig,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub authentication: Authentication,
    pub connection: ConnectionConfig,
    pub aws_storage: AwsS3Storage,
    pub backup_plan: BackupPlan,
}

impl AppState {
    pub fn load_from_env() -> Self {
        Self {
            authentication: Authentication::load_from_env(),
            connection: ConnectionConfig::load_env().unwrap(),
            aws_storage: AwsS3Storage::load_from_env().unwrap(),
            backup_plan: BackupPlan::load_from_env(),
        }
    }

    pub fn check_auth(&self, user: &str, pass: &str) -> bool {
        self.authentication.check(user, pass)
    }

    pub fn check_admin(&self, token: &str) -> bool {
        self.authentication.check_admin(token)
    }

    pub async fn to_graph(&self) -> Result<Graph, anyhow::Error> {
        self.connection.to_graph().await
    }

    pub fn should_backup_world(&self, world_id: &Uuid) -> bool {
        self.backup_plan.should_backup_world(world_id)
    }

    pub async fn aws_client(&self) -> Client {
        self.aws_storage.client().await
    }
}
