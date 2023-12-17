#[derive(Clone, Debug, serde::Deserialize)]
pub struct Buffer {
    #[serde(default)]
    #[serde(rename = "byteLength")]
    pub byte_length: usize,

    pub name: Option<String>,
    pub uri: Option<String>,

    #[serde(default)]
    pub extensions: Option<extensions::BufferExtensions>,
}

pub mod extensions {
    #[derive(Clone, Debug, serde::Deserialize)]
    pub struct BufferExtensions {}
}
