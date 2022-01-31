mod find_meta;

#[derive(Debug)]
pub struct Service;

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
