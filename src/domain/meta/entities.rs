use frunk::LabelledGeneric;

#[derive(Debug, LabelledGeneric)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
