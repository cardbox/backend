#[derive(Clone, Copy, Default)]
pub struct Generator;

impl crate::contracts::Generator for Generator {
    fn secure_token(&self, length: u8) -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length as usize)
            .map(char::from)
            .collect()
    }
}
