use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BackupPlan {
    world_id: Option<Uuid>,
}

impl BackupPlan {
    pub fn load_from_env() -> Self {
        let world_id = match std::env::var("BACKUP_WORLD_ID") {
            Ok(it) => Uuid::parse_str(&it).ok(),
            Err(_) => None,
        };

        Self { world_id }
    }

    pub fn should_backup_world(&self, world_id: &Uuid) -> bool {
        self.world_id.as_ref().eq(&Some(world_id))
    }
}
