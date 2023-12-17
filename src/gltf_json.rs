pub mod accessor;
pub mod animation;
pub mod asset;
pub mod buffer;
pub mod buffer_view;
pub mod camera;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod scene;
pub mod skin;
pub mod texture;

#[doc(inline)]
pub use accessor::Accessor;
#[doc(inline)]
pub use animation::Animation;
#[doc(inline)]
pub use asset::Asset;
#[doc(inline)]
pub use buffer::Buffer;
#[doc(inline)]
pub use buffer_view::BufferView;
#[doc(inline)]
pub use camera::Camera;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use mesh::{Mesh, Primitive};
#[doc(inline)]
pub use node::Node;
#[doc(inline)]
pub use scene::Scene;
#[doc(inline)]
pub use skin::Skin;
#[doc(inline)]
pub use texture::Texture;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Extension {
    KHR_lights_common,
    KHR_lights_punctual,
    KHR_materials_ior,
    KHR_materials_unlit,
    KHR_materials_emissive_strength,
    KHR_materials_transmission,
    KHR_materials_volume,
    KHR_materials_iridescence,
    KHR_materials_sheen,
    KHR_materials_clearcoat,
    KHR_materials_variants,
    KHR_materials_specular,
    KHR_materials_pbrSpecularGlossiness,
    KHR_xmp,
    KHR_texture_transform,
    KHR_texture_basisu,
    KHR_draco_mesh_compression,
    KHR_mesh_quantization,
    EXT_meshopt_compression,
    EXT_lights_image_based,
    Unknown(String),
}

impl<'de> serde::de::Deserialize<'de> for Extension {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Extension;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "any exetsnion string")
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use Extension::*;
                Ok(match value {
                    "KHR_lights_common" => KHR_lights_common,
                    "KHR_lights_punctual" => KHR_lights_punctual,
                    "KHR_materials_ior" => KHR_materials_ior,
                    "KHR_materials_unlit" => KHR_materials_unlit,
                    "KHR_materials_emissive_strength" => KHR_materials_emissive_strength,
                    "KHR_materials_transmission" => KHR_materials_transmission,
                    "KHR_materials_volume" => KHR_materials_volume,
                    "KHR_materials_iridescence" => KHR_materials_iridescence,
                    "KHR_materials_sheen" => KHR_materials_sheen,
                    "KHR_materials_clearcoat" => KHR_materials_clearcoat,
                    "KHR_materials_variants" => KHR_materials_variants,
                    "KHR_materials_specular" => KHR_materials_specular,
                    "KHR_materials_pbrSpecularGlossiness" => KHR_materials_pbrSpecularGlossiness,
                    "KHR_xmp" => KHR_xmp,
                    "KHR_texture_transform" => KHR_texture_transform,
                    "KHR_texture_basisu" => KHR_texture_basisu,
                    "KHR_draco_mesh_compression" => KHR_draco_mesh_compression,
                    "KHR_mesh_quantization" => KHR_mesh_quantization,
                    "EXT_meshopt_compression" => EXT_meshopt_compression,
                    "EXT_lights_image_based" => EXT_lights_image_based,

                    _ => Unknown(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Gltf {
    #[serde(default)]
    pub accessors: Vec<Accessor>,
    #[serde(default)]
    pub animations: Vec<Animation>,
    pub asset: Asset,
    #[serde(default)]
    pub buffers: Vec<Buffer>,
    #[serde(default)]
    #[serde(rename = "bufferViews")]
    pub buffer_views: Vec<BufferView>,
    pub scene: Option<usize>,
    #[serde(default)]
    #[serde(rename = "extensionsUsed")]
    pub extensions_used: Vec<Extension>,
    #[serde(default)]
    #[serde(rename = "extensionsRequired")]
    pub extensions_required: Vec<Extension>,
    #[serde(default)]
    pub cameras: Vec<Camera>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub materials: Vec<Material>,
    #[serde(default)]
    pub meshes: Vec<Mesh>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub samplers: Vec<texture::Sampler>,
    #[serde(default)]
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub skins: Vec<Skin>,
    #[serde(default)]
    pub textures: Vec<Texture>,
    #[serde(default)]
    pub extensions: Option<extensions::GltfExtensions>,
}

#[derive(Debug)]
pub enum GltfError {
    Io(std::io::Error),
    JsonParseError(serde_json::Error),
    PathExtensionNotGltf,
}

impl std::fmt::Display for GltfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GltfError::Io(err) => f.write_fmt(format_args!("{err}")),
            GltfError::JsonParseError(err) => f.write_fmt(format_args!("{err}")),
            GltfError::PathExtensionNotGltf => {
                f.write_str("The extension for the file in Gltf::from_path is not gltf")
            }
        }
    }
}

impl Gltf {
    #[inline]
    pub fn from_path<P>(gltf_path: P) -> Result<Self, GltfError>
    where
        P: AsRef<std::path::Path>,
    {
        let path = gltf_path.as_ref();
        match path.extension() {
            Some(ext) if ext == "gltf" => {
                let file_data = std::fs::read_to_string(path).map_err(|err| GltfError::Io(err))?;
                Self::from_gltf_str(file_data.as_str())
            }

            /*
                TODO: handle glb
            */
            _ => Err(GltfError::PathExtensionNotGltf),
        }
    }

    #[inline]
    pub fn from_gltf_reader<R: std::io::Read>(json: R) -> Result<Self, GltfError> {
        serde_json::from_reader(json).map_err(|err| GltfError::JsonParseError(err))
    }

    #[inline]
    pub fn from_gltf_bytes<S: AsRef<[u8]>>(json_bytes: S) -> Result<Self, GltfError> {
        serde_json::from_slice(json_bytes.as_ref()).map_err(|err| GltfError::JsonParseError(err))
    }

    #[inline]
    pub fn from_gltf_str<S: AsRef<str>>(json: S) -> Result<Self, GltfError> {
        serde_json::from_str(json.as_ref()).map_err(|err| GltfError::JsonParseError(err))
    }
}

pub mod extensions {
    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct GltfExtensions {}
}
