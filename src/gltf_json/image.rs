#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageMimeType {
    Jpeg,
    Png,
    Extension(extensions::ImageMimeTypeExtensions),
    Unknown(String),
}

pub const VALID_IMAGE_MIME_TYPES: &[&str] = &["image/jpeg", "image/png"];

impl<'de> serde::de::Deserialize<'de> for ImageMimeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageMimeType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "any of: {:?}", VALID_IMAGE_MIME_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use ImageMimeType::*;
                match value {
                    "image/jpeg" => Ok(Jpeg),
                    "image/png" => Ok(Png),

                    _ => match serde_json::from_str::<extensions::ImageMimeTypeExtensions>(value) {
                        Ok(it) => Ok(Extension(it)),
                        _ => Ok(Unknown(value.to_owned())),
                    },
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Image {
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<ImageMimeType>,
    pub name: Option<String>,
    pub uri: Option<String>,
    #[serde(default)]
    pub extensions: Option<extensions::ImageExtensions>,
}

pub mod extensions {
    pub const VALID_IMAGE_MIME_TYPE_EXTENSIONS: &[&str] = &["image/ktx2"];

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum ImageMimeTypeExtensions {
        Ktx2,
    }

    impl<'de> serde::de::Deserialize<'de> for ImageMimeTypeExtensions {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = ImageMimeTypeExtensions;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "any of: {:?}", VALID_IMAGE_MIME_TYPE_EXTENSIONS)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    use ImageMimeTypeExtensions::*;
                    match value {
                        "image/ktx2" => Ok(Ktx2),

                        _ => Err(serde::de::Error::custom(
                            "Not a valid ImageMimeTypeExtension",
                        )),
                    }
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    #[derive(Default, Clone, Debug, serde::Deserialize)]
    pub struct ImageExtensions {}
}
