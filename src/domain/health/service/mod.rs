mod get_health;

#[derive(Debug)]
pub struct Service;

impl Service {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}
