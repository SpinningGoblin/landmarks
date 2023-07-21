use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Authentication {
    passwords: HashMap<String, String>,
    admin_token: String,
}

impl Authentication {
    pub fn load_from_env() -> Self {
        let mut passwords: HashMap<String, String> = HashMap::new();
        let admin_token = std::env::var("ADMIN_TOKEN").unwrap();
        passwords.insert(
            "derrick".to_string(),
            std::env::var("DERRICK_PASSWORD").unwrap(),
        );
        passwords.insert(
            "jenny".to_string(),
            std::env::var("JENNY_PASSWORD").unwrap(),
        );

        Self {
            passwords,
            admin_token,
        }
    }

    pub fn check(&self, user: &str, pass: &str) -> bool {
        if let Some(stored) = self.passwords.get(user) {
            pass.eq(stored)
        } else {
            false
        }
    }

    pub fn check_admin(&self, key: &str) -> bool {
        key.eq(&self.admin_token)
    }
}
