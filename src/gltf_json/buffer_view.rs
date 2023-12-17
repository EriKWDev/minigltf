pub const ARRAY_BUFFER: u32 = 34_962;
pub const ELEMENT_ARRAY_BUFFER: u32 = 34_963;

pub const VALID_TARGETS: &[u32] = &[ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    ArrayBuffer,
    ElementArrayBuffer,
}

impl<'de> serde::de::Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Target;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_TARGETS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use Target::*;
                match value as u32 {
                    ARRAY_BUFFER => Ok(ArrayBuffer),
                    ELEMENT_ARRAY_BUFFER => Ok(ElementArrayBuffer),

                    _ => Err(serde::de::Error::custom("A valid Target")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct BufferView {
    pub buffer: usize,

    #[serde(rename = "byteLength")]
    pub byte_length: usize,

    #[serde(default)]
    #[serde(rename = "byteOffset")]
    pub byte_offset: usize,

    #[serde(rename = "byteStride")]
    pub byte_stride: Option<usize>,

    pub name: Option<String>,

    pub target: Option<Target>,

    #[serde(default)]
    pub extensions: Option<extensions::BufferViewExtensions>,
}

pub mod extensions {
    #[derive(Clone, Debug, serde::Deserialize)]
    pub struct BufferViewExtensions {}
}
