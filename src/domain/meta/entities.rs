use frunk::LabelledGeneric;

#[derive(Debug, LabelledGeneric)]
pub struct Meta {
    pub version: String,
    pub build_hash: String,
    pub build_date: String,
    pub config: Config,
}

#[derive(Debug, LabelledGeneric)]
pub struct Config {
    pub env: String,
    pub base_url: String,
    pub port: u16,
}
