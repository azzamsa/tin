#[derive(Debug)]
pub struct Mailer {}

impl Default for Mailer {
    fn default() -> Self {
        Self::new()
    }
}

impl Mailer {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
    pub async fn send(&self, _email: &str) -> Result<(), crate::Error> {
        Ok(())
    }
}
