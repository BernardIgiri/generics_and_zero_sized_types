use std::{collections::HashMap};
use std::io::{self, Write};

use crate::encryption::EncryptionMethod;

pub struct PasswordVault<T: EncryptionMethod> {
    encryption_method: T,
    encrypted_data: Vec<u8>
}

pub struct UnlockedPasswordVault<T: EncryptionMethod> {
    encryption_method: T,
    master_password: String,
    passwords: HashMap<String, String>
}

impl<T: EncryptionMethod> PasswordVault<T> {
    pub fn new(encryption_method: T, encrypted_data:Vec<u8>) -> Self {
        PasswordVault {
            encrypted_data,
            encryption_method
        }
    }

    pub fn unlock(self, master_password: String) -> io::Result<UnlockedPasswordVault<T>> {
        let passwords = 
            if self.encrypted_data.is_empty() {
                Ok(HashMap::new())
            } else {
                let mut decrypted_data = Vec::new();
                let mut decrypted_output = io::Cursor::new(&mut decrypted_data);
                let mut input = io::Cursor::new(self.encrypted_data);
                self.encryption_method.decrypt(
                    &master_password,
                    &mut input,
                    &mut decrypted_output,
                )?;
                bincode::deserialize(decrypted_output.get_ref().as_slice())
            }.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(UnlockedPasswordVault {
            encryption_method: self.encryption_method,
            master_password,
            passwords,
        })
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.encrypted_data.as_slice())
    }
}


impl<T: EncryptionMethod> UnlockedPasswordVault<T> {
    pub fn lock(self) -> io::Result<PasswordVault<T>> {
        let data = bincode::serialize(&self.passwords).unwrap();
        let input = io::Cursor::new(&data);
        let mut encrypted_data = Vec::new();
        let mut output = io::Cursor::new(&mut encrypted_data);
        self.encryption_method.encrypt(&self.master_password, input, &mut output)?;
        Ok(PasswordVault::new(self.encryption_method, encrypted_data))
    }

    pub fn list_users(&self) -> impl Iterator<Item = &String> {
        self.passwords.keys()
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }

    pub fn get_password(&self, username: &str) -> Option<&String> {
        self.passwords.get(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xor_encryption::XorEncryption;

    const MASTER_PASSWORD: &str = "password123";
    const PASSWORDS: [(&str, &str); 2] = [
        ("user1", "somepassword"),
        ("user2", "correcthorsebatterystaple"),
    ];

    fn get_saved_data<T: EncryptionMethod>(vault: &PasswordVault<T>) -> Vec<u8> {
        let mut data = Vec::new();
        let mut writer = io::Cursor::new(&mut data);
        vault.save(&mut writer).unwrap();
        data
    }

    #[test]
    fn password_vault_locks_without_error() {
        let data = Vec::new();
        let vault = PasswordVault::new(XorEncryption, data);
        assert_eq!(get_saved_data(&vault).is_empty(), true);
        let mut vault = vault.unlock(MASTER_PASSWORD.into()).unwrap();
        vault.add_password(PASSWORDS[0].0.into(),PASSWORDS[0].1.into());
        vault.add_password(PASSWORDS[1].0.into(),PASSWORDS[1].1.into());
        let passwords: Vec<&String> = vault.list_users().collect();
        assert_eq!(passwords.len(), 2);
        let vault = vault.lock().unwrap();
        assert_eq!(get_saved_data(&vault).is_empty(), false);
    }

    #[test]
    fn password_vault_doesnt_alter_passwords_on_unlock() {
        let data = Vec::new();
        let vault = PasswordVault::new(XorEncryption, data);
        let mut vault = vault.unlock(MASTER_PASSWORD.into()).unwrap();
        vault.add_password(PASSWORDS[0].0.into(),PASSWORDS[0].1.into());
        vault.add_password(PASSWORDS[1].0.into(),PASSWORDS[1].1.into());
        let vault = vault.lock().unwrap();
        let vault = vault.unlock(MASTER_PASSWORD.into()).unwrap();
        assert_eq!(vault.get_password(PASSWORDS[0].0).unwrap(), PASSWORDS[0].1);
    }
}
