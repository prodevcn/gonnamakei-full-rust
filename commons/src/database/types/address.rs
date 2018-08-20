use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use arcstr::ArcStr;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use solana_sdk::pubkey::Pubkey;

use crate::database::traits::{DBNormalize, DBNormalizeResult};
use crate::database::types::{DBUuid, DBUuidType};
use crate::error::{AppError, AppResult};

/// A base58 address.
/// This type is stored as a string because it is used more in that form..
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Address(ArcStr);

impl Address {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn parse_str(text: &str) -> AppResult<Address> {
        check_address(text)?;
        Ok(Address(text.to_string().into()))
    }

    pub fn parse_arcstr(text: &ArcStr) -> AppResult<Address> {
        check_address(text.as_str())?;
        Ok(Address(text.clone()))
    }

    // METHODS ----------------------------------------------------------------

    pub fn as_string(&self) -> &ArcStr {
        &self.0
    }

    pub fn to_uuid(&self) -> DBUuid {
        DBUuid::new_with_content(self.0.as_str(), DBUuidType::Address).unwrap()
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct UuidVisitor;
        impl<'de> Visitor<'de> for UuidVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a base58 string with 44 characters")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match check_address(v.as_str()) {
                    Ok(_) => Ok(Address(v.into())),
                    Err(e) => Err(E::custom(format!(
                        "Incorrect value for an Address: {:?}",
                        e
                    ))),
                }
            }
        }

        deserializer.deserialize_string(UuidVisitor)
    }
}

impl From<Pubkey> for Address {
    fn from(v: Pubkey) -> Self {
        Self(format!("{}", v).into())
    }
}

impl From<Address> for Pubkey {
    fn from(v: Address) -> Self {
        Pubkey::from_str(v.0.as_str()).unwrap()
    }
}

impl FromStr for Address {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_address(s).map(|_| Address(s.into()))
    }
}

impl TryFrom<DBUuid> for Address {
    type Error = AppError;

    fn try_from(value: DBUuid) -> Result<Self, Self::Error> {
        if value.kind() == DBUuidType::Address {
            Ok(Address(
                value
                    .as_string()
                    .trim_start_matches(DBUuidType::Address.prefix_str())
                    .into(),
            ))
        } else {
            Err(AppError::new(arcstr::literal!(
                "address::decoding::invalid_uuid_type"
            )))
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl DBNormalize for Address {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn check_address(s: &str) -> AppResult<()> {
    if Pubkey::from_str(s).is_err() {
        return Err(AppError::new(arcstr::literal!(
            "address::decoding::invalid_address"
        )));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let address = Address::from_str("5UqVVVKPjorxZrxDu1H8hq1drrVBYY8gvH4FEQFGiFPd")
            .expect("The from_str must succeed");
        let address_ok = Address("5UqVVVKPjorxZrxDu1H8hq1drrVBYY8gvH4FEQFGiFPd".into());
        assert_eq!(address, address_ok);
    }

    #[test]
    fn test_serialization() {
        let address = Address("5UqVVVKPjorxZrxDu1H8hq1drrVBYY8gvH4FEQFGiFPd".into());
        let serialization = serde_json::to_string(&address).unwrap();

        assert_eq!(
            serialization,
            "\"5UqVVVKPjorxZrxDu1H8hq1drrVBYY8gvH4FEQFGiFPd\""
        );

        let deserialization: Address = serde_json::from_str(&serialization).unwrap();
        assert_eq!(deserialization, address);
    }
}
