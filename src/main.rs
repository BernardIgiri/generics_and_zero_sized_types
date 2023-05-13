mod naive_solution;
mod encryption;
mod with_encryption;
mod xor_encryption;

fn main() {
    {
        let vault = naive_solution::PasswordVault::new("password123".to_owned());
        let mut vault = vault.unlock("password123".to_owned());
        vault.list_passwords();
        vault.add_password("bob".into(), "apple".into());
        vault.lock();
    }
    {
        let data = Vec::new();
        let vault = with_encryption::PasswordVault::new(xor_encryption::XorEncryption, data);
        let mut vault = vault.unlock("password123".into()).unwrap();
        let users:Vec<&String> = vault.list_users().collect();
        users.len();
        vault.add_password("bob".into(), "apple".into());
        vault.get_password("bob".into());
        let vault = vault.lock().unwrap();
        let mut save_data = Vec::new();
        let mut writer = std::io::Cursor::new(&mut save_data);
        vault.save(&mut writer).unwrap();
    }
}
