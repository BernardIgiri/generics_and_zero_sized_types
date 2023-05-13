use crate::encryption::EncryptionMethod;
use std::io::{Read, Write};
use std::io;

pub struct XorEncryption;

impl EncryptionMethod for XorEncryption {
    fn encrypt<R: Read, W: Write>(&self, password: &str, mut reader: R, writer: &mut W) -> io::Result<()> {
        let password_bytes = password.as_bytes();
        let mut password_index = 0;

        let mut buf = [0u8; 1024];
        loop {
            let count = reader.read(&mut buf)?;
            if count == 0 {
                break;
            }
            for i in 0..count {
                buf[i] ^= password_bytes[password_index];
                password_index = (password_index + 1) % password_bytes.len();
            }
            writer.write_all(&buf[..count])?;
        }
        Ok(())
    }

    fn decrypt<R: Read, W: Write>(&self, password: &str, mut reader: R, mut writer: &mut W) -> io::Result<()> {
        self.encrypt(password, &mut reader, &mut writer)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const PASSWORD:&str = "password123";
    const PLAIN_TEXT:&[u8] = "hello".as_bytes();
    const ENCRYPTED:&[u8] = &[24u8, 4u8, 31u8, 31u8, 24u8];

    #[test]
    fn encrypt() {
        let data = PLAIN_TEXT.clone();
        let mut result:Vec<u8> = Vec::new();
        let input = io::Cursor::new(data);
        let mut output = io::Cursor::new(&mut result);
        let encryption = XorEncryption;
        encryption.encrypt(PASSWORD, input, &mut output).unwrap();
        assert_eq!(result, ENCRYPTED);
    }

    #[test]
    fn decrypt() {
        let data = ENCRYPTED.clone();
        let mut result:Vec<u8> = Vec::new();
        let input = io::Cursor::new(data);
        let mut output = io::Cursor::new(&mut result);
        let encryption = XorEncryption;
        encryption.decrypt(PASSWORD, input, &mut output).unwrap();
        assert_eq!(result, PLAIN_TEXT);
    }
}
