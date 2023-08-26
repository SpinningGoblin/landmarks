use aws_sdk_s3::Client;
use landmarks_core::{config::neo4j::ConnectionConfig, worlds::World};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{backup_plan::BackupPlan, blob_storage::AwsS3Storage};

#[derive(Clone, Debug)]
pub struct AppState {
    pub connection: ConnectionConfig,
    pub aws_storage: AwsS3Storage,
    pub backup_plan: BackupPlan,
    pub last_backed_up_date: Option<OffsetDateTime>,
}

impl AppState {
    pub fn load_from_env() -> Self {
        Self {
            connection: ConnectionConfig::load_env().unwrap(),
            aws_storage: AwsS3Storage::load_from_env().unwrap(),
            backup_plan: BackupPlan::load_from_env(),
            last_backed_up_date: None,
        }
    }

    pub fn world_to_backup(&self) -> Option<Uuid> {
        self.backup_plan.world_to_backup().cloned()
    }

    pub fn update_last_backed_up(&mut self, date: OffsetDateTime) {
        self.last_backed_up_date = Some(date);
    }

    pub fn need_to_backup_world(&self, world: &World) -> bool {
        let Some(last_synced) = self.last_backed_up_date else {
            return true;
        };

        match world.metadata.updated_at {
            Some(updated_at) => updated_at.gt(&last_synced),
            None => false,
        }
    }

    pub fn connection_config(&self) -> &ConnectionConfig {
        &self.connection
    }

    pub async fn aws_client(&self) -> Client {
        self.aws_storage.client().await
    }
}
