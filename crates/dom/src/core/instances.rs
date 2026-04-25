use crate::data_types;

#[derive(Debug, Clone)]
pub enum Material {
    SmoothPlastic = 0,
    Plastic = 1,
    Brick = 2,
    Wood = 3,
    Grass = 4,
    Ice = 5,
    Sand = 6,
    Metal = 7,
    Aluminum = 8,
    Rust = 9,
    Neon = 10,
    WoodPlanks = 11,
    Marble = 12,
    Slate = 13,
    Concrete = 14,
    Granite = 15,
    Pebble = 16,
    Cobblestone = 17,
    DiamondPlate = 18,
    Fabric = 19,
}

impl Material {
    pub fn from_u64(v: u64) -> Self {
        match v {
            0 => Material::SmoothPlastic,
            1 => Material::Plastic,
            2 => Material::Brick,
            3 => Material::Wood,
            4 => Material::Grass,
            5 => Material::Ice,
            6 => Material::Sand,
            7 => Material::Metal,
            8 => Material::Aluminum,
            9 => Material::Rust,
            10 => Material::Neon,
            11 => Material::WoodPlanks,
            12 => Material::Marble,
            13 => Material::Slate,
            14 => Material::Concrete,
            15 => Material::Granite,
            16 => Material::Pebble,
            17 => Material::Cobblestone,
            18 => Material::DiamondPlate,
            19 => Material::Fabric,
            _ => Material::Plastic,
        }
    }
}

// lighting
#[derive(Debug, Clone)]
pub struct Lighting {
    pub clock_time: f32,
    pub latitude: f32,
    pub ambient: f32,
    pub global_shadow: bool,
    pub sun_colour: data_types::Color3,
    pub sun_intensity: f32,
}

#[derive(Debug, Clone)]
pub struct BaseLight {
    pub light_colour: data_types::Color3,
    pub intensity: f32,
    pub range: f32,
}

#[derive(Debug, Clone)]
pub struct PointLight {
    pub base_light: BaseLight,
    pub cast_shadows: bool,
}

#[derive(Debug, Clone)]
pub struct SpotLight {
    pub base_light: BaseLight,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub cast_shadows: bool,
}

// misc
#[derive(Debug, Clone)]
pub struct Script {
    pub unknown: [u8; 9],
}

#[derive(Debug, Clone)]
pub struct ScriptHandle {
    pub unknown: [u8; 15],
}
