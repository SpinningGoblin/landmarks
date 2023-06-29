use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Authentication {
    passwords: HashMap<String, String>,
}

impl Authentication {
    pub fn load_from_env() -> Self {
        let mut passwords: HashMap<String, String> = HashMap::new();
        passwords.insert(
            "derrick".to_string(),
            std::env::var("DERRICK_PASSWORD").unwrap(),
        );
        passwords.insert(
            "jenny".to_string(),
            std::env::var("JENNY_PASSWORD").unwrap(),
        );

        Self { passwords }
    }

    pub fn check(&self, user: &str, pass: &str) -> bool {
        if let Some(stored) = self.passwords.get(user) {
            pass.eq(stored)
        } else {
            false
        }
    }
}
