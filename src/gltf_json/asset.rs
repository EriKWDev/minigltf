#[derive(Clone, Debug, serde::Deserialize)]
pub struct Asset {
    pub copyright: Option<String>,
    pub generator: Option<String>,
    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,
    pub version: String,
    #[serde(default)]
    pub extensions: Option<extensions::AssetExtensions>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}

pub mod extensions {
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct AssetExtensions {}
}
