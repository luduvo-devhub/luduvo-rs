use crate::{data_types, instances};

#[derive(Debug, Clone)]
pub enum ComponentType {
    Position = 0x0001,
    Rotation = 0x0002,
    Size = 0x0003,
    Name = 0x0004,

    Physics = 0x0014,
    Anchored = 0x0015,
    Locked = 0x0016,
    Color3 = 0x001E,
    Transparency = 0x001F,
    Material = 0x0020,
    SpawnPoints = 0x0032,
    NetworkId = 0x003C,
    Lighting = 0x006E,
    ChildOfPair = 0x005A,

    ScriptHandle = 0x0047,
    Script = 0x0048,

    BoxShapeType = 0x000A,
    SphereShapeType = 0x000B,
    CylinderShapeType = 0x000C,
    ConeShapeType = 0x000D,
    WedgeShapeType = 0x000E,

    PointLight = 0x0064,
    SpotLight = 0x0065,
}

#[derive(Debug, Clone)]
pub struct Physics {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
}

#[derive(Debug, Clone)]
pub struct EntityId {
    pub entity_id: u32,
    pub entity_gen: u32,
}

#[derive(Debug, Clone)]
pub struct Entry<T> {
    pub entity_id: EntityId,
    pub value: T,
}

#[derive(Debug, Clone)]
pub enum EntityRawValue<VS> {
    VS(VS),
    U8(u8),
}

#[derive(Debug, Clone)]
pub struct EntityRaw<VS> {
    pub entity_id: EntityId,
    pub value: EntityRawValue<VS>,
}

#[derive(Debug, Clone)]
pub enum RecordEntries {
    EntityIds(Vec<EntityId>),

    Script(Vec<Entry<instances::Script>>),
    ScriptHandle(Vec<Entry<instances::ScriptHandle>>),

    Vec3(Vec<Entry<data_types::Vec3>>),
    Color3(Vec<Entry<data_types::Color3>>),
    Physics(Vec<Entry<Physics>>),
    Quat(Vec<Entry<data_types::Quat>>),
    Name(Vec<Entry<data_types::String>>),
    Lighting(Vec<Entry<instances::Lighting>>),
    PointLight(Vec<Entry<instances::PointLight>>),
    SpotLight(Vec<Entry<instances::SpotLight>>),

    F32(Vec<Entry<f32>>),
    U64(Vec<Entry<u64>>),

    ChildOfPair(Vec<Entry<EntityId>>),
    Anchored(Vec<Entry<u8>>),

    Raw(Vec<EntityRaw<Vec<u8>>>),
}

#[derive(Debug, Clone)]
pub struct Record {
    pub component_type: ComponentType,
    pub value_size: u16,
    pub number_of_entries: u32,
    pub entries: RecordEntries,
}

impl Record {
    pub fn new(component_type: ComponentType, value_size: u16, number_of_entries: u32) -> Self {
        let entries = match (value_size, &component_type) {
            (0, ComponentType::Script) => RecordEntries::Script(vec![]),

            (0, ComponentType::ScriptHandle) => RecordEntries::ScriptHandle(vec![]),

            (0, _) => RecordEntries::EntityIds(vec![]),

            (_, ComponentType::Position | ComponentType::Size) => RecordEntries::Vec3(vec![]),

            (_, ComponentType::Color3) => RecordEntries::Color3(vec![]),
            (_, ComponentType::Physics) => RecordEntries::Physics(vec![]),
            (_, ComponentType::Rotation) => RecordEntries::Quat(vec![]),
            (_, ComponentType::Name) => RecordEntries::Name(vec![]),
            (_, ComponentType::Lighting) => RecordEntries::Lighting(vec![]),
            (_, ComponentType::PointLight) => RecordEntries::PointLight(vec![]),
            (_, ComponentType::SpotLight) => RecordEntries::SpotLight(vec![]),

            (_, ComponentType::ChildOfPair) => RecordEntries::ChildOfPair(vec![]),

            (4, _) => RecordEntries::F32(vec![]),
            (8, _) => RecordEntries::U64(vec![]),

            (_, ComponentType::Anchored) => RecordEntries::Anchored(vec![]),

            _ => RecordEntries::Raw(vec![]),
        };

        Self {
            component_type,
            value_size,
            number_of_entries,
            entries,
        }
    }
}
