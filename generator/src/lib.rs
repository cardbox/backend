#[derive(Clone)]
pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }
}

impl cardbox_core::generator::Generator for Generator {
    fn secure_token(&self, length: u8) -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length as usize)
            .collect()
    }
}
