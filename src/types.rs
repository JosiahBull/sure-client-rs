use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::ops::Deref;
use uuid::Uuid;

/// Macro to create a simple newtype wrapper for strings
macro_rules! newtype_string {
    ($(#[$attr:meta])* $vis:vis $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $vis struct $name(String);

        impl $name {
            /// Create a new instance
            pub fn new<T: Into<String>>(value: T) -> Self {
                Self(value.into())
            }

            /// Get the inner string as a str
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

/// Macro to create a newtype wrapper for UUIDs
macro_rules! newtype_uuid {
    ($(#[$attr:meta])* $vis:vis $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $vis struct $name(Uuid);

        impl $name {
            /// Create a new instance from a UUID
            pub const fn new(value: Uuid) -> Self {
                Self(value)
            }

            /// Parse from a string
            pub fn parse(value: &str) -> Result<Self, uuid::Error> {
                Ok(Self(Uuid::parse_str(value)?))
            }

            /// Get the inner UUID
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }
    };
}

newtype_string!(
    /// Bearer token for authentication (JWT)
    pub BearerToken
);

newtype_string!(
    /// API key for authentication via X-Api-Key header
    pub ApiKey
);

/// Authentication method for the Sure API
///
/// The API supports two authentication methods:
/// - Bearer token (JWT) via Authorization header
/// - API key via X-Api-Key header
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Auth {
    /// Bearer token authentication (Authorization: Bearer <token>)
    Bearer(BearerToken),
    /// API key authentication (X-Api-Key: <key>)
    ApiKey(ApiKey),
}

impl Auth {
    /// Create Bearer token authentication
    pub fn bearer<T: Into<String>>(token: T) -> Self {
        Self::Bearer(BearerToken::new(token))
    }

    /// Create API key authentication
    pub fn api_key<T: Into<String>>(key: T) -> Self {
        Self::ApiKey(ApiKey::new(key))
    }
}

impl From<BearerToken> for Auth {
    fn from(token: BearerToken) -> Self {
        Self::Bearer(token)
    }
}

impl From<ApiKey> for Auth {
    fn from(key: ApiKey) -> Self {
        Self::ApiKey(key)
    }
}

newtype_uuid!(
    /// Account identifier
    pub AccountId
);

newtype_uuid!(
    /// Category identifier
    pub CategoryId
);

newtype_uuid!(
    /// Merchant identifier
    pub MerchantId
);

newtype_uuid!(
    /// Tag identifier
    pub TagId
);

newtype_uuid!(
    /// Transaction identifier
    pub TransactionId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearer_token() {
        let token = BearerToken::new("test_token");
        assert_eq!(token.as_str(), "test_token");
        assert_eq!(&*token, "test_token");
        assert_eq!(token.to_string(), "test_token");
    }

    #[test]
    fn test_uuid_types() {
        let uuid = Uuid::new_v4();
        let account_id = AccountId::new(uuid);
        assert_eq!(account_id.as_uuid(), &uuid);
        assert_eq!(account_id.to_string(), uuid.to_string());

        let parsed = AccountId::parse(&uuid.to_string())
            .expect("UUID parsing should succeed for valid UUID string");
        assert_eq!(parsed, account_id);
    }
}
