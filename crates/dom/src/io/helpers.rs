use std::io::Write;

use crate::errors::DecodeError;
use crate::records::{self, ComponentType, EntityId, Entry, Record, RecordEntries};
use crate::{data_types, errors, instances};

pub struct Cursor<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], errors::DecodeError> {
        if self.offset + N > self.data.len() {
            return Err(errors::DecodeError::UnexpectedEOF(N));
        }

        let mut buf = [0u8; N];

        buf.copy_from_slice(&self.data[self.offset..self.offset + N]);

        self.offset += N;

        Ok(buf)
    }

    pub fn read_u16(&mut self) -> Result<u16, DecodeError> {
        Ok(u16::from_le_bytes(self.read_exact()?))
    }

    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(self.read_exact()?))
    }
}

pub fn read_f32(cursor: &mut Cursor) -> Result<f32, DecodeError> {
    Ok(f32::from_le_bytes(cursor.read_exact()?))
}

pub fn read_entity_id(cursor: &mut Cursor) -> Result<EntityId, DecodeError> {
    Ok(EntityId {
        entity_id: cursor.read_u32()?,
        entity_gen: cursor.read_u32()?,
    })
}

pub fn read_vec3(cursor: &mut Cursor) -> Result<data_types::Vec3, DecodeError> {
    Ok(data_types::Vec3 {
        x: read_f32(cursor)?,
        y: read_f32(cursor)?,
        z: read_f32(cursor)?,
    })
}

pub fn read_quat(cursor: &mut Cursor) -> Result<data_types::Quat, DecodeError> {
    Ok(data_types::Quat {
        x: read_f32(cursor)?,
        y: read_f32(cursor)?,
        z: read_f32(cursor)?,
        w: read_f32(cursor)?,
    })
}

pub fn read_color3(cursor: &mut Cursor) -> Result<data_types::Color3, DecodeError> {
    Ok(data_types::Color3 {
        r: read_f32(cursor)?,
        g: read_f32(cursor)?,
        b: read_f32(cursor)?,
    })
}

pub fn read_u8(cursor: &mut Cursor) -> Result<u8, DecodeError> {
    Ok(cursor.read_exact::<1>()?[0])
}

pub fn read_bool(cursor: &mut Cursor) -> Result<bool, DecodeError> {
    Ok(read_u8(cursor)? != 0)
}

pub fn read_string(cursor: &mut Cursor) -> Result<data_types::String, DecodeError> {
    let bytes = cursor.read_exact::<64>()?;

    let len = bytes.iter().position(|&b| b == 0).unwrap_or(64);
    let slice = &bytes[..len];

    let text = std::str::from_utf8(slice).unwrap_or("");

    Ok(data_types::String {
        text: fixedstr::str64::from(text),
    })
}

pub fn read_lighting(cursor: &mut Cursor) -> Result<instances::Lighting, DecodeError> {
    Ok(instances::Lighting {
        clock_time: read_f32(cursor)?,
        latitude: read_f32(cursor)?,
        ambient: read_f32(cursor)?,
        global_shadow: read_bool(cursor)?,
        sun_colour: read_color3(cursor)?,
        sun_intensity: read_f32(cursor)?,
    })
}

pub fn read_point_light(cursor: &mut Cursor) -> Result<instances::PointLight, DecodeError> {
    Ok(instances::PointLight {
        base_light: instances::BaseLight {
            light_colour: read_color3(cursor)?,
            intensity: read_f32(cursor)?,
            range: read_f32(cursor)?,
        },
        cast_shadows: read_bool(cursor)?,
    })
}

pub fn read_spot_light(cursor: &mut Cursor) -> Result<instances::SpotLight, DecodeError> {
    Ok(instances::SpotLight {
        base_light: instances::BaseLight {
            light_colour: read_color3(cursor)?,
            intensity: read_f32(cursor)?,
            range: read_f32(cursor)?,
        },

        inner_angle: read_f32(cursor)?,
        outer_angle: read_f32(cursor)?,
        cast_shadows: read_bool(cursor)?,
    })
}

pub fn component_from_u16(v: u16) -> Result<ComponentType, DecodeError> {
    Ok(match v {
        0x0001 => ComponentType::Position,
        0x0002 => ComponentType::Rotation,
        0x0003 => ComponentType::Size,
        0x0004 => ComponentType::Name,

        0x0014 => ComponentType::Physics,
        0x0015 => ComponentType::Anchored,
        0x0016 => ComponentType::Locked,
        0x001E => ComponentType::Color3,
        0x001F => ComponentType::Transparency,
        0x0020 => ComponentType::Material,
        0x0032 => ComponentType::SpawnPoints,
        0x003C => ComponentType::NetworkId,
        0x006E => ComponentType::Lighting,
        0x005A => ComponentType::ChildOfPair,

        0x0047 => ComponentType::ScriptHandle,
        0x0048 => ComponentType::Script,

        0x000A => ComponentType::BoxShapeType,
        0x000B => ComponentType::SphereShapeType,
        0x000C => ComponentType::CylinderShapeType,
        0x000D => ComponentType::ConeShapeType,
        0x000E => ComponentType::WedgeShapeType,

        0x0064 => ComponentType::PointLight,
        0x0065 => ComponentType::SpotLight,

        _ => return Err(DecodeError::InvalidComponentType(v)),
    })
}

pub fn read_record(cursor: &mut Cursor) -> Result<records::Record, DecodeError> {
    let component_raw = cursor.read_u16()?;
    let value_size = cursor.read_u16()?;
    let number_of_entries = cursor.read_u32()?;

    let component_type = component_from_u16(component_raw)?;
    let mut record = records::Record::new(component_type.clone(), value_size, number_of_entries);

    match &mut record.entries {
        RecordEntries::EntityIds(vec) => {
            for _ in 0..number_of_entries {
                vec.push(read_entity_id(cursor)?);
            }
        }

        RecordEntries::Script(vec) => {
            for _ in 0..number_of_entries {
                let entity_id = read_entity_id(cursor)?;
                let unknown = cursor.read_exact::<9>()?;

                vec.push(Entry {
                    entity_id,
                    value: instances::Script { unknown },
                });
            }
        }

        RecordEntries::ScriptHandle(vec) => {
            for _ in 0..number_of_entries {
                let entity_id = read_entity_id(cursor)?;
                let unknown = cursor.read_exact::<15>()?;

                vec.push(Entry {
                    entity_id,
                    value: instances::ScriptHandle { unknown },
                });
            }
        }

        RecordEntries::Vec3(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_vec3(cursor)?,
                });
            }
        }

        RecordEntries::Quat(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_quat(cursor)?,
                });
            }
        }

        RecordEntries::Color3(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_color3(cursor)?,
                });
            }
        }

        RecordEntries::Physics(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,

                    value: records::Physics {
                        friction: read_f32(cursor)?,
                        restitution: read_f32(cursor)?,
                        density: read_f32(cursor)?,
                    },
                });
            }
        }

        RecordEntries::F32(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_f32(cursor)?,
                });
            }
        }

        RecordEntries::U64(vec) => {
            for _ in 0..number_of_entries {
                let entity_id = read_entity_id(cursor)?;
                let value = u64::from_le_bytes(cursor.read_exact()?);

                vec.push(Entry { entity_id, value });
            }
        }

        RecordEntries::Raw(vec) => {
            for _ in 0..number_of_entries {
                let entity_id = read_entity_id(cursor)?;
                let mut data = vec![0u8; value_size as usize];

                for i in 0..value_size as usize {
                    data[i] = cursor.read_exact::<1>()?[0];
                }

                vec.push(records::EntityRaw {
                    entity_id,
                    value: records::EntityRawValue::VS(data),
                });
            }
        }

        RecordEntries::Name(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_string(cursor)?,
                });
            }
        }

        RecordEntries::Lighting(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_lighting(cursor)?,
                });
            }
        }

        RecordEntries::PointLight(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_point_light(cursor)?,
                });
            }
        }

        RecordEntries::SpotLight(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_spot_light(cursor)?,
                });
            }
        }

        RecordEntries::ChildOfPair(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_entity_id(cursor)?,
                });
            }
        }

        RecordEntries::Anchored(vec) => {
            for _ in 0..number_of_entries {
                vec.push(Entry {
                    entity_id: read_entity_id(cursor)?,
                    value: read_u8(cursor)?,
                });
            }
        }
    }

    Ok(record)
}

pub fn write_record<W: Write>(writer: &mut W, record: &Record) -> std::io::Result<()> {
    writer.write_all(&(record.component_type.clone() as u16).to_le_bytes())?;
    writer.write_all(&record.value_size.to_le_bytes())?;
    writer.write_all(&record.number_of_entries.to_le_bytes())?;

    match &record.entries {
        RecordEntries::Vec3(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.x.to_le_bytes())?;
                writer.write_all(&e.value.y.to_le_bytes())?;
                writer.write_all(&e.value.z.to_le_bytes())?;
            }
        }

        RecordEntries::Quat(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.x.to_le_bytes())?;
                writer.write_all(&e.value.y.to_le_bytes())?;
                writer.write_all(&e.value.z.to_le_bytes())?;
                writer.write_all(&e.value.w.to_le_bytes())?;
            }
        }

        RecordEntries::Color3(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.r.to_le_bytes())?;
                writer.write_all(&e.value.g.to_le_bytes())?;
                writer.write_all(&e.value.b.to_le_bytes())?;
            }
        }

        RecordEntries::F32(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.to_le_bytes())?;
            }
        }

        RecordEntries::U64(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.to_le_bytes())?;
            }
        }

        RecordEntries::Name(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(e.value.text.as_bytes())?;
            }
        }

        RecordEntries::EntityIds(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_gen.to_le_bytes())?;
            }
        }

        RecordEntries::Physics(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.friction.to_le_bytes())?;
                writer.write_all(&e.value.restitution.to_le_bytes())?;
                writer.write_all(&e.value.density.to_le_bytes())?;
            }
        }

        RecordEntries::Anchored(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&[e.value])?;
            }
        }

        RecordEntries::ChildOfPair(entries) => {
            for e in entries {
                writer.write_all(&e.entity_id.entity_id.to_le_bytes())?;
                writer.write_all(&e.entity_id.entity_gen.to_le_bytes())?;

                writer.write_all(&e.value.entity_id.to_le_bytes())?;
                writer.write_all(&e.value.entity_gen.to_le_bytes())?;
            }
        }

        _ => {}
    }

    Ok(())
}
