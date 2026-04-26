# File



## fields

- header: `FileHeader`
- records: `Vec<records :: Record>`

## methods

### new
**returns**
- `Self`

### from
**arguments**
- data: `&[u8]`

**returns**
- `Result<File, DecodeError>`

### encode
**arguments**
- writer: `&mut W`

**returns**
- `std :: io :: Result<()>`

### default
**returns**
- `Self`

