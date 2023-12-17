#[derive(Clone, Debug, serde::Deserialize)]
pub struct Skin {
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<usize>,
    pub joints: Vec<usize>,
    pub name: Option<String>,
    pub skeleton: Option<usize>,
    #[serde(default)]
    pub extensions: Option<extensions::SkinExtensions>,
}

pub mod extensions {
    #[derive(Default, Debug, Copy, Clone, serde::Deserialize)]
    pub struct SkinExtensions {}
}
