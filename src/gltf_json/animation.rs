pub const VALID_INTERPOLATIONS: &[&str] = &["LINEAR", "STEP", "CUBICSPLINE"];

pub const VALID_PROPERTIES: &[&str] = &["translation", "rotation", "scale", "weights"];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]

pub enum Interpolation {
    Linear,
    Step,
    CubicSpline,
}

impl Default for Interpolation {
    #[inline]
    fn default() -> Self {
        Self::Linear
    }
}

impl<'de> serde::de::Deserialize<'de> for Interpolation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Interpolation;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_INTERPOLATIONS)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use Interpolation::*;
                match value {
                    "LINEAR" => Ok(Linear),
                    "STEP" => Ok(Step),
                    "CUBICSPLINE" => Ok(CubicSpline),

                    _ => Err(serde::de::Error::custom("Expected a valid Interpolation")),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Property {
    Translation,
    Rotation,
    Scale,
    MorphTargetWeights,
}

impl<'de> serde::de::Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Property;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_PROPERTIES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use Property::*;
                match value {
                    "translation" => Ok(Translation),
                    "rotation" => Ok(Rotation),
                    "scale" => Ok(Scale),
                    "weights" => Ok(MorphTargetWeights),

                    _ => Err(serde::de::Error::custom("A valid Interpolation")),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Animation {
    #[serde(default)]
    pub extensions: Option<extensions::AnimationExtension>,
    pub channels: Vec<Channel>,
    pub name: Option<String>,
    pub samplers: Vec<Sampler>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Channel {
    pub sampler: usize,
    pub target: Target,
    #[serde(default)]
    pub extensions: Option<extensions::ChannelExtension>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Target {
    #[serde(default)]
    pub extensions: Option<extensions::TargetExtension>,
    pub node: usize,
    pub path: Property,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Sampler {
    #[serde(default)]
    pub extensions: Option<extensions::SamplerExtension>,
    pub input: usize,
    #[serde(default)]
    pub interpolation: Interpolation,
    pub output: usize,
}

pub mod extensions {
    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct AnimationExtension {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct ChannelExtension {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct TargetExtension {}

    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct SamplerExtension {}
}
