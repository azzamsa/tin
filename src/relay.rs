use async_graphql::static_assertions::_core::fmt::Formatter;
use uuid::Uuid;

/// Base64 invalid states, used by `Base64Cursor`.
pub enum Base64CursorError {
    /// Invalid cursor. This can happen if the base64 string is valid, but its contents don't
    /// conform to the `name:index` pattern.
    Invalid,
    /// Decoding error. If this happens, the string isn't valid base64.
    DecodeError(base64::DecodeError),
}

impl std::fmt::Display for Base64CursorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid cursor")
    }
}

/// Base64 cursor implementation
pub struct Base64Cursor {
    name: &'static str,
    index: Uuid,
}
impl Base64Cursor {
    pub fn new(index: Uuid) -> Self {
        Self {
            name: "Cursor",
            index,
        }
    }

    /// Returns a base64 string representation of the cursor
    pub fn encode(&self) -> String {
        base64::encode_config(
            format!("{}:{}", self.name, self.index),
            base64::URL_SAFE_NO_PAD,
        )
    }

    /// Decodes a base64 string into a cursor result
    pub fn decode(s: &str) -> Result<Self, Base64CursorError> {
        let bytes = base64::decode_config(s, base64::URL_SAFE_NO_PAD)
            .map_err(Base64CursorError::DecodeError)?;

        let cursor = String::from_utf8(bytes).map_err(|_| Base64CursorError::Invalid)?;
        let index = cursor
            .split(':')
            .last()
            .map(|s| s.parse::<Uuid>())
            .ok_or(Base64CursorError::Invalid)?
            .map_err(|_| Base64CursorError::Invalid)?;

        Ok(Self::new(index))
    }
}

impl std::fmt::Debug for Base64Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.index)
    }
}

impl From<Base64Cursor> for Uuid {
    fn from(cursor: Base64Cursor) -> Self {
        cursor.index
    }
}
