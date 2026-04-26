#[cfg(feature = "friends")]
pub use super::users::friends::{
    Client as FriendsClient, Config as FriendsConfig, Error as FriendsError, Friends,
};

#[cfg(feature = "profile")]
pub use super::users::profile::{
    Client as ProfileClient, Config as ProfileConfig, Error as ProfileError, Profile,
};

#[cfg(feature = "query")]
pub use super::users::query::{
    Client as QueryClient, Config as QueryConfig, Error as QueryError, Query,
};

#[cfg(feature = "places")]
pub use super::places::{
    Client as PlacesClient, Config as PlacesConfig, Error as PlacesError, Place, Places,
};
