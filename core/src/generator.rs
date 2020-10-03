pub trait Generator {
    fn secure_token(&self, length: u8) -> String;
}
