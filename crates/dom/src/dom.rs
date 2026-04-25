use std::collections::HashMap;

use crate::data_types::*;
use crate::errors::DecodeError;
use crate::file::{File, FileHeader};
use crate::instances::*;
use crate::records::{ComponentType, EntityId, Entry, Physics, Record, RecordEntries};

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u64,
    pub generation: u32,

    pub parent: Option<u64>,
    pub children: Vec<u64>,

    pub components: Components,

    pub is_real: bool,
}

#[derive(Debug, Clone)]
pub struct Components {
    pub position: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub size: Option<Vec3>,
    pub name: Option<String>,
    pub color: Option<Color3>,
    pub transparency: Option<f32>,
    pub material: Option<u64>,
    pub shape: Option<ComponentType>,
    pub physics: Option<Physics>,
    pub anchored: Option<bool>,
}

impl Components {
    pub fn none() -> Components {
        Components {
            position: None,
            rotation: None,
            size: None,
            name: None,
            color: None,
            transparency: None,
            material: None,
            shape: None,
            physics: None,
            anchored: None,
        }
    }
}

impl Default for Components {
    fn default() -> Components {
        Components {
            position: Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),

            rotation: Some(Quat {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }),

            size: Some(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            name: None,

            color: Some(Color3 {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }),

            transparency: Some(0.0),
            material: Some(Material::Plastic as u64),
            shape: Some(ComponentType::BoxShapeType),

            physics: Some(Physics {
                friction: 0.5,
                restitution: 0.3,
                density: 100.0,
            }),

            anchored: Some(true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dom {
    pub entities: HashMap<u64, Entity>,
    pub root_entities: Vec<u64>,
    pub raw_records: Vec<Record>,
}

impl Dom {
    pub fn from_file(file: &File) -> Self {
        let mut raw_records = vec![];
        let mut entities: HashMap<u64, Entity> = HashMap::new();

        fn ensure_entity(entities: &mut HashMap<u64, Entity>, id: &EntityId) {
            let key = id.entity_id as u64;
            entities.entry(key).or_insert(Entity {
                id: key,
                generation: id.entity_gen,
                parent: None,
                children: vec![],
                components: Components::default(),
                is_real: false,
            });
        }

        fn ensure_entity_skeleton(entities: &mut HashMap<u64, Entity>, id: &EntityId) {
            let key = id.entity_id as u64;
            entities.entry(key).or_insert(Entity {
                id: key,
                generation: id.entity_gen,
                parent: None,
                children: vec![],
                components: Components::none(),
                is_real: false,
            });
        }

        for record in &file.records {
            match &record.entries {
                RecordEntries::Vec3(_)
                | RecordEntries::Quat(_)
                | RecordEntries::Color3(_)
                | RecordEntries::F32(_)
                | RecordEntries::U64(_)
                | RecordEntries::Name(_)
                | RecordEntries::Physics(_)
                | RecordEntries::Anchored(_)
                | RecordEntries::EntityIds(_) => {}

                _ => {
                    raw_records.push(record.clone());

                    continue;
                }
            }

            match &record.entries {
                RecordEntries::Vec3(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();

                        match record.component_type {
                            ComponentType::Position => {
                                ent.components.position = Some(e.value.clone())
                            }
                            ComponentType::Size => ent.components.size = Some(e.value.clone()),
                            _ => {}
                        }
                    }
                }

                RecordEntries::Quat(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();
                        ent.components.rotation = Some(e.value.clone());
                    }
                }

                RecordEntries::Color3(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();
                        ent.components.color = Some(e.value.clone());
                    }
                }

                RecordEntries::F32(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();

                        if let ComponentType::Transparency = record.component_type {
                            ent.components.transparency = Some(e.value);
                        }
                    }
                }

                RecordEntries::U64(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();

                        if let ComponentType::Material = record.component_type {
                            ent.components.material = Some(e.value);
                        }
                    }
                }

                RecordEntries::Name(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();
                        ent.components.name = Some(e.value.clone());
                    }
                }

                RecordEntries::EntityIds(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, e);
                        let ent = entities.get_mut(&(e.entity_id as u64)).unwrap();
                        match record.component_type {
                            ComponentType::BoxShapeType
                            | ComponentType::SphereShapeType
                            | ComponentType::CylinderShapeType
                            | ComponentType::ConeShapeType
                            | ComponentType::WedgeShapeType => {
                                ent.components.shape = Some(record.component_type.clone());
                                ent.is_real = true;
                            }
                            _ => {}
                        }
                    }
                }

                RecordEntries::Physics(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();
                        ent.components.physics = Some(e.value.clone());
                    }
                }

                RecordEntries::Anchored(entries) => {
                    for e in entries {
                        ensure_entity(&mut entities, &e.entity_id);
                        let ent = entities.get_mut(&(e.entity_id.entity_id as u64)).unwrap();
                        ent.components.anchored = Some(e.value != 0);
                    }
                }

                RecordEntries::U64(_) => {}
                RecordEntries::Raw(_) => {}
                RecordEntries::Script(_) | RecordEntries::ScriptHandle(_) => {}
                RecordEntries::Lighting(_)
                | RecordEntries::PointLight(_)
                | RecordEntries::SpotLight(_) => {}
                RecordEntries::SpotLight(_) => {}
                RecordEntries::PointLight(_) => {}
                RecordEntries::Lighting(_) => {}
                RecordEntries::ChildOfPair(_) => {}

                _ => {}
            }
        }

        for record in &file.records {
            if let RecordEntries::ChildOfPair(entries) = &record.entries {
                for e in entries {
                    let child = e.entity_id.entity_id as u64;
                    let parent = e.value.entity_id as u64;

                    ensure_entity(&mut entities, &e.entity_id);
                    ensure_entity_skeleton(&mut entities, &e.value);

                    if let Some(child_ent) = entities.get_mut(&child) {
                        if parent == 0 || parent == u64::MAX {
                            child_ent.parent = None;
                        } else {
                            child_ent.parent = Some(parent);
                        }
                    }

                    if parent != 0
                        && parent != u64::MAX
                        && let Some(parent_ent) = entities.get_mut(&parent)
                        && !parent_ent.children.contains(&child)
                    {
                        parent_ent.children.push(child);
                    }
                }
            }
        }

        // collect roots
        let root_entities = entities
            .values()
            .filter(|e| e.parent.is_none())
            .map(|e| e.id)
            .collect();

        Dom {
            entities,
            root_entities,
            raw_records,
        }
    }

    pub fn create_entity(&mut self, id: u64) {
        self.entities.insert(
            id,
            Entity {
                id,
                generation: 0,
                parent: None,
                children: vec![],
                components: Components::default(),
                is_real: true,
            },
        );

        if !self.root_entities.contains(&id) {
            self.root_entities.push(id);
        }
    }

    pub fn set_parent(&mut self, child: u64, parent: u64) {
        if let Some(child_ent) = self.entities.get_mut(&child) {
            child_ent.parent = Some(parent);
        }

        if let Some(parent_ent) = self.entities.get_mut(&parent)
            && !parent_ent.children.contains(&child)
        {
            parent_ent.children.push(child);
        }

        self.root_entities.retain(|&id| id != child);
    }

    pub fn remove_entity(&mut self, id: u64) {
        if let Some(ent) = self.entities.remove(&id) {
            for child in ent.children {
                self.remove_entity(child);
            }
        }

        self.root_entities.retain(|&x| x != id);

        for e in self.entities.values_mut() {
            e.children.retain(|&c| c != id);
        }
    }

    pub fn set_position(&mut self, id: u64, pos: Vec3) {
        if let Some(ent) = self.entities.get_mut(&id) {
            ent.components.position = Some(pos);
        }
    }

    pub fn set_name(&mut self, id: u64, name: String) {
        if let Some(ent) = self.entities.get_mut(&id) {
            ent.components.name = Some(name);
        }
    }

    pub fn to_file(&self) -> Result<File, DecodeError> {
        let mut records: Vec<Record> = vec![];

        let mut positions = vec![];
        let mut rotations = vec![];
        let mut sizes = vec![];
        let mut names = vec![];
        let mut colors = vec![];
        let mut transparencies = vec![];
        let mut materials = vec![];
        let mut child_of = vec![];
        let mut shapes = vec![];
        let mut physics = vec![];
        let mut anchored = vec![];

        for entity in self.entities.values().filter(|e| e.is_real) {
            let eid = EntityId {
                entity_id: entity.id as u32,
                entity_gen: entity.generation,
            };

            if let Some(v) = &entity.components.position {
                positions.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = &entity.components.rotation {
                rotations.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = &entity.components.size {
                sizes.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = &entity.components.name {
                names.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = &entity.components.color {
                colors.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = entity.components.transparency {
                transparencies.push(Entry {
                    entity_id: eid.clone(),
                    value: v,
                });
            }

            if let Some(v) = entity.components.material {
                materials.push(Entry {
                    entity_id: eid.clone(),
                    value: v,
                });
            }

            if let Some(parent) = entity.parent {
                child_of.push(Entry {
                    entity_id: eid.clone(),

                    value: EntityId {
                        entity_id: parent as u32,
                        entity_gen: 0,
                    },
                });
            }

            if let Some(shape) = &entity.components.shape {
                shapes.push(EntityId {
                    entity_id: entity.id as u32,
                    entity_gen: entity.generation,
                });
            }

            if let Some(v) = &entity.components.physics {
                physics.push(Entry {
                    entity_id: eid.clone(),
                    value: v.clone(),
                });
            }

            if let Some(v) = entity.components.anchored {
                anchored.push(Entry {
                    entity_id: eid.clone(),
                    value: if v { 1u8 } else { 0u8 },
                });
            }
        }

        // records.extend(
        //     self.raw_records
        //         .iter()
        //         .filter(|r| {
        //             matches!(
        //                 r.component_type,
        //                 ComponentType::Script
        //                     | ComponentType::ScriptHandle
        //                     | ComponentType::Lighting
        //                     | ComponentType::PointLight
        //                     | ComponentType::SpotLight
        //             )
        //         })
        //         .cloned(),
        // );

        if !positions.is_empty() {
            records.push(Record {
                component_type: ComponentType::Position,
                value_size: 12,
                number_of_entries: positions.len() as u32,
                entries: RecordEntries::Vec3(positions),
            });
        }

        if !rotations.is_empty() {
            records.push(Record {
                component_type: ComponentType::Rotation,
                value_size: 16,
                number_of_entries: rotations.len() as u32,
                entries: RecordEntries::Quat(rotations),
            });
        }

        if !sizes.is_empty() {
            records.push(Record {
                component_type: ComponentType::Size,
                value_size: 12,
                number_of_entries: sizes.len() as u32,
                entries: RecordEntries::Vec3(sizes),
            });
        }

        if !names.is_empty() {
            records.push(Record {
                component_type: ComponentType::Name,
                value_size: 64,
                number_of_entries: names.len() as u32,
                entries: RecordEntries::Name(names),
            });
        }

        if !colors.is_empty() {
            records.push(Record {
                component_type: ComponentType::Color3,
                value_size: 12,
                number_of_entries: colors.len() as u32,
                entries: RecordEntries::Color3(colors),
            });
        }

        if !transparencies.is_empty() {
            records.push(Record {
                component_type: ComponentType::Transparency,
                value_size: 4,
                number_of_entries: transparencies.len() as u32,
                entries: RecordEntries::F32(transparencies),
            });
        }

        if !materials.is_empty() {
            records.push(Record {
                component_type: ComponentType::Material,
                value_size: 8,
                number_of_entries: materials.len() as u32,
                entries: RecordEntries::U64(materials),
            });
        }

        if !child_of.is_empty() {
            records.push(Record {
                component_type: ComponentType::ChildOfPair,
                value_size: 8,
                number_of_entries: child_of.len() as u32,
                entries: RecordEntries::ChildOfPair(child_of),
            });
        }

        if !shapes.is_empty() {
            records.push(Record {
                component_type: ComponentType::BoxShapeType,
                value_size: 8,
                number_of_entries: shapes.len() as u32,
                entries: RecordEntries::EntityIds(shapes),
            });
        }

        if !physics.is_empty() {
            records.push(Record {
                component_type: ComponentType::Physics,
                value_size: std::mem::size_of::<Physics>() as u16,
                number_of_entries: physics.len() as u32,
                entries: RecordEntries::Physics(physics),
            });
        }

        if !anchored.is_empty() {
            records.push(Record {
                component_type: ComponentType::Anchored,
                value_size: 1,
                number_of_entries: anchored.len() as u32,
                entries: RecordEntries::Anchored(anchored),
            });
        }

        let mut missing_fields: Vec<std::string::String> = vec![];

        for entity in self.entities.values().filter(|e| e.is_real) {
            let missing = self.validate_entity(entity);

            if !missing.is_empty() {
                missing_fields.push(format!(
                    "Entity {} missing: {}",
                    entity.id,
                    missing.join(", ")
                ));
            }
        }

        if !missing_fields.is_empty() {
            return Err(DecodeError::MissingComponentFields(missing_fields));
        }

        Ok(File {
            header: FileHeader {
                magic: *b"LSCN",
                version: 7,
                record_count: records.len() as u16,
            },
            records,
        })
    }

    fn validate_entity(&self, entity: &Entity) -> Vec<std::string::String> {
        let mut missing = Vec::new();

        if entity.components.position.is_none() {
            missing.push("position".to_string());
        }

        if entity.components.rotation.is_none() {
            missing.push("rotation".to_string());
        }

        if entity.components.size.is_none() {
            missing.push("size".to_string());
        }

        if entity.components.color.is_none() {
            missing.push("color".to_string());
        }

        if entity.components.transparency.is_none() {
            missing.push("transparency".to_string());
        }

        if entity.components.material.is_none() {
            missing.push("material".to_string());
        }

        if entity.components.shape.is_none() {
            missing.push("shape".to_string());
        }

        if entity.components.physics.is_none() {
            missing.push("physics".to_string());
        }

        if entity.components.anchored.is_none() {
            missing.push("anchored".to_string());
        }

        missing
    }
}
