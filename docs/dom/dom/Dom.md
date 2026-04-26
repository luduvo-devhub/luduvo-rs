# Dom



## fields

- entities: `HashMap<u64, Entity>`
- root_entities: `Vec<u64>`
- raw_records: `Vec<Record>`

## methods

### from_file
**arguments**
- file: `&File`

**returns**
- `Self`

### create_entity
**arguments**
- id: `u64`

### set_parent
**arguments**
- child: `u64`
- parent: `u64`

### remove_entity
**arguments**
- id: `u64`

### set_position
**arguments**
- id: `u64`
- pos: `Vec3`

### set_name
**arguments**
- id: `u64`
- name: `String`

### to_file
**returns**
- `Result<File, DecodeError>`

### validate_entity
**arguments**
- entity: `&Entity`

**returns**
- `Vec<std :: string :: String>`

