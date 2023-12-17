pub const BYTE: u32 = 5120;
pub const UNSIGNED_BYTE: u32 = 5121;
pub const SHORT: u32 = 5122;
pub const UNSIGNED_SHORT: u32 = 5123;
pub const UNSIGNED_INT: u32 = 5125;
pub const FLOAT: u32 = 5126;

pub const VALID_COMPONENT_TYPES: &[u32] = &[
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
    FLOAT,
];

pub const VALID_ACCESSOR_TYPES: &[&str] =
    &["SCALAR", "VEC2", "VEC3", "VEC4", "MAT2", "MAT3", "MAT4"];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ComponentType {
    I8,
    U8,
    I16,
    U16,
    U32,
    F32,
}

impl ComponentType {
    pub fn size(&self) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::I16 | Self::U16 => 2,
            Self::F32 | Self::U32 => 4,
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for ComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComponentType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_COMPONENT_TYPES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use ComponentType::*;
                match value as u32 {
                    BYTE => Ok(I8),
                    UNSIGNED_BYTE => Ok(U8),
                    SHORT => Ok(I16),
                    UNSIGNED_SHORT => Ok(U16),
                    UNSIGNED_INT => Ok(U32),
                    FLOAT => Ok(F32),

                    _ => Err(serde::de::Error::custom("A valid ComponentType")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl<'de> serde::de::Deserialize<'de> for AccessorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AccessorType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_ACCESSOR_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use AccessorType::*;
                match value {
                    "SCALAR" => Ok(Scalar),
                    "VEC2" => Ok(Vec2),
                    "VEC3" => Ok(Vec3),
                    "VEC4" => Ok(Vec4),
                    "MAT2" => Ok(Mat2),
                    "MAT3" => Ok(Mat3),
                    "MAT4" => Ok(Mat4),

                    _ => Err(serde::de::Error::custom("Not a valid AccessorType")),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl AccessorType {
    pub fn multiplicity(&self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Accessor {
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[serde(default)]
    #[serde(rename = "byteOffset")]
    pub byte_offset: usize,
    pub count: usize,
    #[serde(rename = "componentType")]
    pub component_type: ComponentType,
    #[serde(rename = "type")]
    pub accessor_type: AccessorType,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub normalized: bool,
    #[serde(default)]
    pub sparse: Option<sparse::Sparse>,
    pub extensions: Option<extensions::AccessorExtensions>,
}

pub mod sparse {
    use super::*;

    #[derive(Clone, Debug, serde::Deserialize)]
    pub struct Indices {
        #[serde(rename = "bufferView")]
        pub buffer_view: usize,
        #[serde(default)]
        #[serde(rename = "byteOffset")]
        pub byte_offset: usize,
        #[serde(rename = "componentType")]
        pub component_type: ComponentType,
        #[serde(default)]
        pub extensions: Option<extensions::IndicesExtension>,
    }

    #[derive(Clone, Debug, serde::Deserialize)]
    pub struct Sparse {
        pub count: usize,
        pub indices: Indices,
        pub values: Values,
        #[serde(default)]
        pub extensions: Option<extensions::SparseExtension>,
    }

    #[derive(Clone, Debug, serde::Deserialize)]
    pub struct Values {
        #[serde(rename = "bufferView")]
        pub buffer_view: usize,
        #[serde(default)]
        #[serde(rename = "byteOffset")]
        pub byte_offset: usize,
        #[serde(default)]
        pub extensions: Option<extensions::ValuesExtension>,
    }
}

pub mod extensions {
    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct AccessorExtensions {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct IndicesExtension {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct SparseExtension {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct ValuesExtension {}
}
