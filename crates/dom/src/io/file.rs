use std::io::Write;

use crate::records::{self, ComponentType, EntityId, Entry, RecordEntries};
use crate::{data_types, errors::DecodeError, io::helpers, instances};

#[derive(Debug, Clone)]
pub struct FileHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub record_count: u16,
}

#[derive(Debug, Clone)]
pub struct File {
    pub header: FileHeader,
    pub records: Vec<records::Record>,
}

impl File {
    pub fn new() -> Self {
        File {
            header: FileHeader {
                magic: *b"LSCN",
                version: 1,
                record_count: 0,
            },

            records: Vec::new(),
        }
    }

    pub fn from(data: &[u8]) -> Result<File, DecodeError> {
        let mut cursor = helpers::Cursor::new(data);

        let magic_bytes = cursor.read_exact::<4>()?;
        let magic = [magic_bytes[0], magic_bytes[1], magic_bytes[2], magic_bytes[3]];

        if &magic != b"LSCN" {
            return Err(DecodeError::InvalidMagic {
                normal: String::from_utf8_lossy(&magic).to_string(),
                bytes: magic,
            });
        }

        let version = cursor.read_u32()?;
        let record_count = cursor.read_u16()?;

        let header = FileHeader {
            magic,
            version,
            record_count,
        };

        let mut records = Vec::with_capacity(record_count as usize);

        for _ in 0..record_count {
            records.push(helpers::read_record(&mut cursor)?);
        }

        Ok(File { header, records })
    }

    pub fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.header.magic)?;
        writer.write_all(&self.header.version.to_le_bytes())?;
        writer.write_all(&self.header.record_count.to_le_bytes())?;

        for record in &self.records {
            helpers::write_record(writer, record)?;
        }

        Ok(())
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
