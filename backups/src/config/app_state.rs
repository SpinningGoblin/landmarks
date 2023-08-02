use aws_sdk_s3::Client;
use landmarks_core::config::neo4j::ConnectionConfig;
use neo4rs::Graph;
use uuid::Uuid;

use super::{backup_plan::BackupPlan, blob_storage::AwsS3Storage};

#[derive(Clone, Debug)]
pub struct AppState {
    pub connection: ConnectionConfig,
    pub aws_storage: AwsS3Storage,
    pub backup_plan: BackupPlan,
}

impl AppState {
    pub fn load_from_env() -> Self {
        Self {
            connection: ConnectionConfig::load_env().unwrap(),
            aws_storage: AwsS3Storage::load_from_env().unwrap(),
            backup_plan: BackupPlan::load_from_env(),
        }
    }

    pub fn world_to_backup(&self) -> Option<Uuid> {
        self.backup_plan.world_to_backup().cloned()
    }

    pub async fn to_graph(&self) -> Result<Graph, anyhow::Error> {
        self.connection
            .to_graph()
            .await
            .map_err(|e| anyhow::Error::new(e))
    }

    pub async fn aws_client(&self) -> Client {
        self.aws_storage.client().await
    }
}
