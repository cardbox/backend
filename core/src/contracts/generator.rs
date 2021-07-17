#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
pub trait Generator: Send + Sync {
    fn secure_token(&self, length: u8) -> String;
}
