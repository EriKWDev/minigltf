#[derive(Clone, Debug, serde::Deserialize)]
pub struct Node {
    pub camera: Option<usize>,
    pub children: Option<Vec<usize>>,
    pub matrix: Option<[f32; 16]>,
    pub mesh: Option<usize>,
    pub name: Option<String>,
    pub scale: Option<[f32; 3]>,
    pub translation: Option<[f32; 3]>,
    pub rotation: Option<UnitQuaternion>,
    pub skin: Option<usize>,
    pub weights: Option<Vec<f32>>,

    #[serde(default)]
    pub extensions: Option<extensions::NodeExtensions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct UnitQuaternion(pub [f32; 4]);

impl Default for UnitQuaternion {
    #[inline]
    fn default() -> Self {
        UnitQuaternion([0.0, 0.0, 0.0, 1.0])
    }
}

pub mod extensions {
    #[derive(Debug, Clone, serde::Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum KHR_lights_punctual_kind {
        Directional,
        Point,
        Spot {
            inner_cone_angle: Option<f32>,
            outer_cone_angle: Option<f32>,
        },
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    #[allow(non_camel_case_types)]
    pub struct KHR_lights_punctual_light {
        pub name: Option<String>,
        pub color: Option<[f32; 3]>,
        pub intensity: Option<f32>,
        pub range: Option<f32>,
        pub kind: KHR_lights_punctual_kind,
    }

    #[derive(Debug, Copy, Clone, serde::Deserialize)]
    #[allow(non_camel_case_types)]
    pub struct KHR_lights_punctual {
        pub light: usize,
    }

    #[derive(Default, Debug, Copy, Clone, serde::Deserialize)]
    pub struct NodeExtensions {
        pub khr_lights_punctual: Option<KHR_lights_punctual>,
    }
}
