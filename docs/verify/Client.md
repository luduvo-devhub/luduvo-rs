# Client



## fields

- rng: `ThreadRng`
- client: `ProfileClient`
- settings: `Settings`

## methods

### new
**arguments**
- settings: `Option<Settings>`

**returns**
- `Self`

### generate_code
**arguments**
- code_complexity: `Option<CodeComplexity>`

**returns**
- `String`

### is_verified
**arguments**
- luduvo_username: `String`
- code: `String`

**returns**
- `Result<bool, Error>`

