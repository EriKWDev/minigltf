#[derive(Clone, Debug, serde::Deserialize)]
pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,

    #[serde(default)]
    pub extensions: Option<extensions::SceneExtensions>,
}

pub mod extensions {
    #[derive(Default, Debug, Copy, Clone, serde::Deserialize)]
    pub struct SceneExtensions {}
}
