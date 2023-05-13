use std::collections::HashMap;

pub struct PasswordVault {
    master_password: String,
    passwords: HashMap<String, String>,
}

pub struct UnlockedPasswordVault {
    vault: PasswordVault,
}

impl PasswordVault {
    pub fn unlock(self, master_password: String) -> UnlockedPasswordVault {
        if master_password.eq(&self.master_password) {
            UnlockedPasswordVault::new(self, master_password)
        } else {
            panic!("incorrect password!");
        }
        
    }
    pub fn new(master_password: String) -> Self {
        PasswordVault {
            master_password,
            passwords: Default::default()
        }
    }
}

impl UnlockedPasswordVault {
    pub fn new(vault: PasswordVault, #[allow(unused)] master_password: String ) -> Self {
        UnlockedPasswordVault { vault }
    }
    pub fn lock(self) -> PasswordVault {
        self.vault
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.vault.passwords
    }

    #[allow(dead_code)]
    pub fn add_password(&mut self, username: String, password: String) {
        self.vault.passwords.insert(username, password);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_unlock_password_vault() {
        let vault: PasswordVault = PasswordVault::new("password123".into());
        let mut vault = vault.unlock("password123".into());
        vault.add_password("user1".into(),"somepassword".into());
        vault.add_password("user2".into(),"correcthorsebatterystaple".into());
        let passwords = vault.list_passwords();
        assert_eq!(passwords.len(), 2);
        #[allow(unused)]
        let mut vault = vault.lock();
    }
}
