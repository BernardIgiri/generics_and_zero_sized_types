use std::io::{Read, Write};

pub trait EncryptionMethod {
    fn encrypt<R: Read, W: Write>(&self, password: &str, input: R, output: &mut W) -> std::io::Result<()>;
    fn decrypt<R: Read, W: Write>(&self, password: &str, input: R, output: &mut W) -> std::io::Result<()>;
}
