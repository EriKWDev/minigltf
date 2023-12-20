pub const NEAREST: u32 = 9728;
pub const LINEAR: u32 = 9729;
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

pub const VALID_MIN_FILTERS: &[u32] = &[
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
];

pub const VALID_MAG_FILTERS: &[u32] = &[NEAREST, LINEAR];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl<'de> serde::de::Deserialize<'de> for MinFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MinFilter;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_MIN_FILTERS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use MinFilter::*;
                match value as u32 {
                    NEAREST => Ok(Nearest),
                    LINEAR => Ok(Linear),
                    NEAREST_MIPMAP_NEAREST => Ok(NearestMipmapNearest),
                    LINEAR_MIPMAP_NEAREST => Ok(LinearMipmapNearest),
                    NEAREST_MIPMAP_LINEAR => Ok(NearestMipmapLinear),
                    LINEAR_MIPMAP_LINEAR => Ok(LinearMipmapLinear),

                    _ => Err(serde::de::Error::custom("A valid MinFilter")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MagFilter {
    Nearest,
    Linear,
}

impl<'de> serde::de::Deserialize<'de> for MagFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MagFilter;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_MAG_FILTERS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use MagFilter::*;
                match value as u32 {
                    NEAREST => Ok(Nearest),
                    LINEAR => Ok(Linear),

                    _ => Err(serde::de::Error::custom("A valid MagFilter")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

pub const CLAMP_TO_EDGE: u32 = 33_071;
pub const MIRRORED_REPEAT: u32 = 33_648;
pub const REPEAT: u32 = 10_497;

pub const VALID_WRAPPING_MODES: &[u32] = &[CLAMP_TO_EDGE, MIRRORED_REPEAT, REPEAT];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WrappingMode {
    ClampToEdge,
    MirroredRepeat,
    Repeat,
}

impl<'de> serde::de::Deserialize<'de> for WrappingMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WrappingMode;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_MAG_FILTERS)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use WrappingMode::*;
                match value as u32 {
                    CLAMP_TO_EDGE => Ok(ClampToEdge),
                    MIRRORED_REPEAT => Ok(MirroredRepeat),
                    REPEAT => Ok(Repeat),

                    _ => Err(serde::de::Error::custom("A valid WrappingMode")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

impl Default for WrappingMode {
    #[inline]
    fn default() -> Self {
        Self::Repeat
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Sampler {
    #[serde(rename = "magFilter")]
    pub mag_filter: Option<MagFilter>,
    #[serde(rename = "minFilter")]
    pub min_filter: Option<MinFilter>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "wrapS")]
    pub wrap_s: WrappingMode,
    #[serde(default)]
    #[serde(rename = "wrapT")]
    pub wrap_t: WrappingMode,
    #[serde(default)]
    pub extensions: Option<extensions::SamplerExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Texture {
    pub name: Option<String>,
    pub sampler: Option<usize>,
    pub source: Option<usize>,
    #[serde(default)]
    pub extensions: Option<extensions::TextureExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Info {
    pub index: usize,

    #[serde(default)]
    #[serde(rename = "texCoord")]
    pub tex_coord: usize,

    #[serde(default)]
    pub extensions: Option<extensions::InfoExtensions>,
}

pub mod extensions {
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct InfoExtensions {
        #[serde(rename = "KHR_texture_transform")]
        pub khr_texture_transform: Option<KHR_Texture_Transform>,
    }

    #[allow(non_camel_case_types)]
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct KHR_Texture_Transform {
        #[serde(default)]
        pub offset: Option<[f32; 2]>,
        #[serde(default)]
        pub scale: Option<[f32; 2]>,
        #[serde(default)]
        pub rotation: Option<f32>,
    }

    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct TextureExtensions {}

    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct SamplerExtensions {}
}
