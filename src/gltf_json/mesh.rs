pub const POINTS: u32 = 0;
pub const LINES: u32 = 1;
pub const LINE_LOOP: u32 = 2;
pub const LINE_STRIP: u32 = 3;
pub const TRIANGLES: u32 = 4;
pub const TRIANGLE_STRIP: u32 = 5;
pub const TRIANGLE_FAN: u32 = 6;
pub const QUADS: u32 = 7;
pub const QUAD_STRIP: u32 = 8;
pub const POLYGON: u32 = 9;

pub const VALID_PRIMITIVE_MODES: &[u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
    QUADS,
    QUAD_STRIP,
    POLYGON,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveMode {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quad,
    QuadStrip,
    Polygon,
}

impl<'de> serde::de::Deserialize<'de> for PrimitiveMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrimitiveMode;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_PRIMITIVE_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use PrimitiveMode::*;
                match value as u32 {
                    POINTS => Ok(Points),
                    LINES => Ok(Lines),
                    LINE_LOOP => Ok(LineLoop),
                    LINE_STRIP => Ok(LineStrip),
                    TRIANGLES => Ok(Triangles),
                    TRIANGLE_STRIP => Ok(TriangleStrip),
                    TRIANGLE_FAN => Ok(TriangleFan),
                    QUADS => Ok(Quad),
                    QUAD_STRIP => Ok(QuadStrip),
                    POLYGON => Ok(Polygon),

                    _ => Err(serde::de::Error::custom("A valid PrimitiveMode")),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

impl Default for PrimitiveMode {
    #[inline]
    fn default() -> Self {
        PrimitiveMode::Triangles
    }
}

#[derive(Copy, Clone, Debug, serde::Deserialize)]
pub struct MorphTarget {
    #[serde(rename = "POSITION")]
    pub positions: Option<usize>,
    #[serde(rename = "NORMAL")]
    pub normals: Option<usize>,
    #[serde(rename = "TANGENT")]
    pub tangents: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    Positions,
    Normals,
    Tangents,
    Colors(usize),
    TexCoords(usize),
    Joints(usize),
    Weights(usize),
    Extension(extensions::AttributeExtensions),
    Unknown(String),
}

impl<'de> serde::de::Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Attribute;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a valid accessor")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use Attribute::*;
                match value.rsplit_once('_') {
                    Some(("COLOR", n)) => Ok(Colors(
                        n.parse().map_err(|_| serde::de::Error::custom(value))?,
                    )),
                    Some(("TEXCOORD", n)) => Ok(TexCoords(
                        n.parse().map_err(|_| serde::de::Error::custom(value))?,
                    )),
                    Some(("JOINTS", n)) => Ok(Joints(
                        n.parse().map_err(|_| serde::de::Error::custom(value))?,
                    )),
                    Some(("WEIGHTS", n)) => Ok(Weights(
                        n.parse().map_err(|_| serde::de::Error::custom(value))?,
                    )),

                    _ => match value {
                        "POSITION" => Ok(Positions),
                        "NORMAL" => Ok(Normals),
                        "TANGENT" => Ok(Tangents),

                        _ => match serde_json::from_str::<extensions::AttributeExtensions>(value) {
                            Ok(it) => Ok(Extension(it)),

                            _ => Ok(Unknown(value.to_owned())),
                        },
                    },
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Mesh {
    pub name: Option<String>,
    pub primitives: Vec<Primitive>,
    pub weights: Option<Vec<f32>>,
    #[serde(default)]
    pub extensions: Option<extensions::MeshExtensions>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Primitive {
    pub attributes: std::collections::HashMap<Attribute, usize>,
    pub indices: Option<usize>,
    pub material: Option<usize>,
    #[serde(default)]
    pub mode: PrimitiveMode,
    pub targets: Option<Vec<MorphTarget>>,
    #[serde(default)]
    pub extensions: Option<extensions::PrimitiveExtensions>,
}

/*
    pub const MAX_PRIMITIVE_JOINTS: usize = 3;
    pub const MAX_PRIMITIVE_WEIGHTS: usize = 1;
    pub const MAX_PRIMITIVE_TEXCOORDS: usize = 1;
    pub const MAX_PRIMITIVE_COLORS: usize = 1;

    #[derive(Debug)]
    pub struct Primitive {
        pub position: Option<usize>,
        pub normal: Option<usize>,
        pub tangent: Option<usize>,

        pub joints: [Option<usize>; MAX_PRIMITIVE_JOINTS],
        pub weights: [Option<usize>; MAX_PRIMITIVE_WEIGHTS],
        pub tex_coord: [Option<usize>; MAX_PRIMITIVE_TEXCOORDS],
        pub color: [Option<usize>; MAX_PRIMITIVE_COLORS],

        pub targets: Vec<std::collections::HashMap<String, usize>>,

        pub mode: Option<PrimitiveMode>,
        pub material: Option<usize>,
        pub indices: Option<usize>,
    }
*/

pub mod extensions {
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum AttributeExtensions {}

    impl<'de> serde::de::Deserialize<'de> for AttributeExtensions {
        #[inline]
        fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            Err(serde::de::Error::custom(""))
        }
    }

    #[derive(Default, Debug, Copy, Clone, serde::Deserialize)]
    pub struct PrimitiveExtensions {}

    #[derive(Default, Debug, Copy, Clone, serde::Deserialize)]
    pub struct MeshExtensions {}
}
