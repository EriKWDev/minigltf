pub const VALID_CAMERA_TYPES: &[&str] = &["perspective", "orthographic"];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

impl<'de> serde::de::Deserialize<'de> for CameraType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CameraType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_CAMERA_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use CameraType::*;
                match value {
                    "perspective" => Ok(Perspective),
                    "orthographic" => Ok(Orthographic),

                    _ => Err(serde::de::Error::custom("A valid CameraType")),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Camera {
    pub name: Option<String>,
    pub orthographic: Option<Orthographic>,
    pub perspective: Option<Perspective>,
    #[serde(rename = "type")]
    pub type_: CameraType,
    #[serde(default)]
    pub extensions: Option<extensions::CameraExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Orthographic {
    pub xmag: f32,
    pub ymag: f32,
    pub zfar: f32,
    pub znear: f32,
    #[serde(default)]
    pub extensions: Option<extensions::OrthographicExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Perspective {
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,
    pub yfov: f32,
    pub zfar: Option<f32>,
    pub znear: f32,

    #[serde(default)]
    pub extensions: Option<extensions::PerspectiveExtensions>,
}

pub mod extensions {
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct CameraExtensions {}
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct OrthographicExtensions {}
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct PerspectiveExtensions {}
}
