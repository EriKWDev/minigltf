pub const VALID_ALPHA_MODES: &[&str] = &["OPAQUE", "MASK", "BLEND"];

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

impl Default for AlphaMode {
    #[inline]
    fn default() -> Self {
        Self::Opaque
    }
}

impl<'de> serde::de::Deserialize<'de> for AlphaMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AlphaMode;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_ALPHA_MODES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use AlphaMode::*;
                match value {
                    "OPAQUE" => Ok(Opaque),
                    "MASK" => Ok(Mask),
                    "BLEND" => Ok(Blend),

                    _ => Err(serde::de::Error::custom("Not a valid AlphaMode")),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Material {
    #[serde(rename = "alphaCutoff")]
    pub alpha_cutoff: Option<AlphaCutoff>,
    #[serde(rename = "alphaMode")]
    pub alpha_mode: AlphaMode,
    #[serde(rename = "doubleSided")]
    pub double_sided: bool,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: PbrMetallicRoughness,
    #[serde(rename = "normalTexture")]
    pub normal_texture: Option<NormalTexture>,
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<OcclusionTexture>,
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: Option<super::texture::Info>,
    #[serde(rename = "emissiveFactor")]
    pub emissive_factor: EmissiveFactor,
    #[serde(default)]
    pub extensions: Option<extensions::MaterialExtensions>,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PbrMetallicRoughness {
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: PbrBaseColorFactor,
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: Option<super::texture::Info>,
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: NormalScaleFactor,
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: NormalScaleFactor,
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: Option<super::texture::Info>,
    #[serde(default)]
    pub extensions: Option<extensions::PbrMetallicRoughnessExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NormalTexture {
    pub index: usize,
    #[serde(default)]
    pub scale: NormalScaleFactor,
    #[serde(default)]
    #[serde(rename = "texCoord")]
    pub tex_coord: usize,
    #[serde(default)]
    pub extensions: Option<extensions::NormalTextureExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct OcclusionTexture {
    pub index: usize,
    #[serde(default)]
    pub strength: OcclusionStrengthFactor,
    #[serde(default)]
    #[serde(rename = "texCoord")]
    pub tex_coord: usize,
    #[serde(default)]
    pub extensions: Option<extensions::OcclusionTextureExtensions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct AlphaCutoff(pub f32);

#[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
pub struct EmissiveFactor(pub [f32; 3]);

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct PbrBaseColorFactor(pub [f32; 4]);

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct NormalScaleFactor(pub f32);

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct OcclusionStrengthFactor(pub f32);

impl Default for AlphaCutoff {
    #[inline]
    fn default() -> Self {
        Self(0.5)
    }
}

impl Default for PbrBaseColorFactor {
    #[inline]
    fn default() -> Self {
        Self([1.0, 1.0, 1.0, 1.0])
    }
}

impl Default for NormalScaleFactor {
    #[inline]
    fn default() -> Self {
        Self(1.0)
    }
}

impl Default for OcclusionStrengthFactor {
    #[inline]
    fn default() -> Self {
        Self(1.0)
    }
}

pub mod extensions {
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct MaterialExtensions {
        #[serde(rename = "KHR_materials_pbrSpecularGlossiness")]
        pub khr_materials_pbr_specular_glossiness: Option<KHR_Materials_PbrSpecularGlossiness>,
    }

    #[allow(non_camel_case_types)]
    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct KHR_Materials_PbrSpecularGlossiness {
        #[serde(default)]
        #[serde(rename = "diffuseFactor")]
        pub diffuse_factor: PbrDiffuseFactor,
        #[serde(rename = "diffuseTexture")]
        pub diffuse_texture: Option<crate::texture::Info>,
        #[serde(default)]
        #[serde(rename = "specularFactor")]
        pub specular_factor: PbrSpecularFactor,
        #[serde(default)]
        #[serde(rename = "glossinessFactor")]
        pub glossiness_factor: GlossinessStrengthFactor,
        #[serde(rename = "specularGlossinessTexture")]
        pub specular_glossiness_texture: Option<crate::texture::Info>,
    }

    #[derive(Clone, Copy, Debug, serde::Deserialize)]
    pub struct GlossinessStrengthFactor(pub f32);

    #[derive(Clone, Copy, Debug, serde::Deserialize)]
    pub struct PbrDiffuseFactor(pub [f32; 4]);

    #[derive(Clone, Copy, Debug, serde::Deserialize)]
    pub struct PbrSpecularFactor(pub [f32; 3]);

    impl Default for GlossinessStrengthFactor {
        #[inline]
        fn default() -> Self {
            Self(0.1)
        }
    }

    impl Default for PbrDiffuseFactor {
        #[inline]
        fn default() -> Self {
            Self([0.0, 0.0, 0.0, 1.0])
        }
    }

    impl Default for PbrSpecularFactor {
        #[inline]
        fn default() -> Self {
            Self([0.0, 0.0, 0.0])
        }
    }

    #[derive(Default, Clone, Copy, Debug, serde::Deserialize)]
    pub struct PbrMetallicRoughnessExtensions {}

    #[derive(Default, Clone, Copy, Debug, serde::Deserialize)]
    pub struct NormalTextureExtensions {}

    #[derive(Default, Clone, Copy, Debug, serde::Deserialize)]
    pub struct OcclusionTextureExtensions {}
}
