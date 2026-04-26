# Cursor



## fields

- data: `&'a [u8]`
- offset: `usize`

## methods

### new
**arguments**
- data: `&'a [u8]`

**returns**
- `Self`

### read_exact
**returns**
- `Result<[u8 ; N], errors :: DecodeError>`

### read_u16
**returns**
- `Result<u16, DecodeError>`

### read_u32
**returns**
- `Result<u32, DecodeError>`

