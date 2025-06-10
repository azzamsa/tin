mod get_meta;

use std::sync::Arc;

use crate::config::Config;

#[derive(Debug)]
pub struct Service {
    config: Arc<Config>,
}

impl Service {
    #[must_use]
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
