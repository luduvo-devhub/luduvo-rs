# Profile

represents a user profile returned by the luduvo api.

## fields

- user_id: `u64` - the users id. this is unique to each profile.
- username: `String` - the users username. this is unique to each profile.
- display_name: `String` - display name shown to other users. when the account is first created, this defaults to [`username`](Self::username). it can be changed by the user at any time.
- bio: `Option<String>` - optional long-form description of the profile.
- status: `Option<String>` - a status code of what the user is currently doing.
- avatar: `ProfileAvatar` - the user's avatar appearance configuration. currently, it is just hex codes for the avatar's limbs.
- equipped_items: `Vec<EquippedItem>` - a list of the user's equipped items.
- badges: `Vec<Badge>` - a list of badge identifiers earned by the user.
- friend_count: `u64` - the total number of friends the user has.
- place_count: `u64` - the total number of owned places the user has.
- item_count: `u64` - the total number of owned items the user has.
- last_active: `Option<u64>` - last active timestamp (in unix seconds). this is a `None` if the user has never logged in.
- member_since: `Option<u64>` - account creation timestamp (in unix seconds).
- allow_joins: `bool` - whether others are allowed to join this user.
- is_owner: `bool` - whether the current viewer owns the resource being viewed.
