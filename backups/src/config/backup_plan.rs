use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BackupPlan {
    world_id: Option<Uuid>,
}

impl BackupPlan {
    pub fn world_to_backup(&self) -> Option<&Uuid> {
        self.world_id.as_ref()
    }

    pub fn load_from_env() -> Self {
        let world_id = match std::env::var("BACKUP_WORLD_ID") {
            Ok(it) => Uuid::parse_str(&it).ok(),
            Err(_) => None,
        };

        Self { world_id }
    }
}
