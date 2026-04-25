use crate::{file::File, io::helpers::Cursor, records::*};

pub struct Writer {
    pub data: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn write_u8(&mut self, v: u8) {
        self.data.push(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.data.extend(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.data.extend(&v.to_le_bytes());
    }

    pub fn write_u64(&mut self, v: u64) {
        self.data.extend(&v.to_le_bytes());
    }

    pub fn write_f32(&mut self, v: f32) {
        self.data.extend(&v.to_le_bytes());
    }

    pub fn write_bytes(&mut self, b: &[u8]) {
        self.data.extend(b);
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl File {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_bytes(&self.header.magic);
        w.write_u32(self.header.version);
        w.write_u16(self.records.len() as u16);

        for record in &self.records {
            w.write_u16(record.component_type.clone() as u16);
            w.write_u16(record.value_size);
            w.write_u32(record.number_of_entries);

            match &record.entries {
                RecordEntries::EntityIds(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id);
                        w.write_u32(e.entity_gen);
                    }
                }

                RecordEntries::Vec3(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.x);
                        w.write_f32(e.value.y);
                        w.write_f32(e.value.z);
                    }
                }

                RecordEntries::Quat(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.x);
                        w.write_f32(e.value.y);
                        w.write_f32(e.value.z);
                        w.write_f32(e.value.w);
                    }
                }

                RecordEntries::Color3(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.r);
                        w.write_f32(e.value.g);
                        w.write_f32(e.value.b);
                    }
                }

                RecordEntries::Name(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        let mut buf = [0u8; 64];
                        let bytes = e.value.text.as_str().as_bytes();
                        let len = bytes.len().min(64);
                        buf[..len].copy_from_slice(&bytes[..len]);

                        w.write_bytes(&buf);
                    }
                }

                RecordEntries::F32(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);
                        w.write_f32(e.value);
                    }
                }

                RecordEntries::U64(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);
                        w.write_u64(e.value);
                    }
                }

                RecordEntries::ChildOfPair(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_u32(e.value.entity_id);
                        w.write_u32(e.value.entity_gen);
                    }
                }

                RecordEntries::Physics(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.friction);
                        w.write_f32(e.value.restitution);
                        w.write_f32(e.value.density);
                    }
                }

                RecordEntries::Lighting(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.clock_time);
                        w.write_f32(e.value.latitude);
                        w.write_f32(e.value.ambient);
                        w.write_u8(e.value.global_shadow as u8);

                        w.write_f32(e.value.sun_colour.r);
                        w.write_f32(e.value.sun_colour.g);
                        w.write_f32(e.value.sun_colour.b);

                        w.write_f32(e.value.sun_intensity);
                    }
                }

                RecordEntries::PointLight(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.base_light.light_colour.r);
                        w.write_f32(e.value.base_light.light_colour.g);
                        w.write_f32(e.value.base_light.light_colour.b);

                        w.write_f32(e.value.base_light.intensity);
                        w.write_f32(e.value.base_light.range);

                        w.write_u8(e.value.cast_shadows as u8);
                    }
                }

                RecordEntries::SpotLight(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        w.write_f32(e.value.base_light.light_colour.r);
                        w.write_f32(e.value.base_light.light_colour.g);
                        w.write_f32(e.value.base_light.light_colour.b);

                        w.write_f32(e.value.base_light.intensity);
                        w.write_f32(e.value.base_light.range);

                        w.write_f32(e.value.inner_angle);
                        w.write_f32(e.value.outer_angle);

                        w.write_u8(e.value.cast_shadows as u8);
                    }
                }

                RecordEntries::Script(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);
                        w.write_bytes(&e.value.unknown);
                    }
                }

                RecordEntries::ScriptHandle(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);
                        w.write_bytes(&e.value.unknown);
                    }
                }

                RecordEntries::Raw(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);

                        match &e.value {
                            EntityRawValue::VS(bytes) => {
                                w.write_bytes(bytes);
                            }
                            EntityRawValue::U8(v) => {
                                w.write_u8(*v);
                            }
                        }
                    }
                }
                
                RecordEntries::Anchored(vec) => {
                    for e in vec {
                        w.write_u32(e.entity_id.entity_id);
                        w.write_u32(e.entity_id.entity_gen);
                
                        w.write_u8(e.value);
                    }
                }
            }
        }

        w.data
    }
}
