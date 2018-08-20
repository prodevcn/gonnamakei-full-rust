use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::io::Cursor;
use std::str::FromStr;

use arcstr::ArcStr;
use bitstream_io::{BigEndian, BitRead, BitReader};
use chrono::{TimeZone, Utc};
use nanoid::nanoid;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use solana_sdk::pubkey::Pubkey;

use enum_derive::EnumList;

use crate::database::traits::{DBNormalize, DBNormalizeResult};
use crate::database::types::DateTime;
use crate::error::{AppError, AppResult};

// Char set used to create the codes.
// We do not use the default because it is not correctly sorted in DB.
const ALPHABET: [char; 64] = [
    '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

lazy_static! {
    static ref CHAR_SET: HashSet<char> = ALPHABET.iter().copied().collect();
    static ref CHAR_SET_REVERSE: HashMap<char, u64> = ALPHABET
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i as u64))
        .collect();
}

/// This type is stored as a string because it is used more in that form than as u128.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DBUuid(ArcStr);

impl DBUuid {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(kind: DBUuidType) -> DBUuid {
        let random_chars = kind.random_chars();

        if kind.contains_date() {
            let prefix = kind.prefix_char_codes();

            Self::new_with_code_and_date(DateTime::now(), nanoid!(random_chars, &ALPHABET), &prefix)
        } else {
            let prefix = kind.prefix_str();
            let mut result = nanoid!(random_chars, &ALPHABET);
            result.insert_str(0, prefix);

            DBUuid(result.into())
        }
    }

    pub fn new_with_content(content: &str, kind: DBUuidType) -> AppResult<DBUuid> {
        let text = format!("{}{}", kind.prefix_str(), content);
        check_nanoid(text.as_str(), Some(kind))?;
        Ok(DBUuid(text.into()))
    }

    pub fn parse_str(text: &str) -> AppResult<DBUuid> {
        check_nanoid(text, None)?;
        Ok(DBUuid(text.to_string().into()))
    }

    pub fn parse_str_type(text: &str, kind: DBUuidType) -> AppResult<DBUuid> {
        check_nanoid(text, Some(kind))?;
        Ok(DBUuid(text.to_string().into()))
    }

    #[cfg(any(feature = "test", test))]
    pub fn new_with_code_for_test(uid: usize, kind: DBUuidType) -> DBUuid {
        Self::new_with_code_and_date_for_test(uid, kind, DateTime::now())
    }

    #[cfg(any(feature = "test", test))]
    pub fn new_with_code_and_date_for_test(uid: usize, kind: DBUuidType, now: DateTime) -> DBUuid {
        let random_chars = kind.random_chars();

        if kind.contains_date() {
            let prefix = kind.prefix_char_codes();

            Self::new_with_code_and_date(
                now,
                format!("{:0>alfa$}", uid, alfa = random_chars),
                &prefix,
            )
        } else {
            let prefix = kind.prefix_str();
            let mut result = format!("{:0>alfa$}", uid, alfa = random_chars);
            result.insert_str(0, prefix);

            DBUuid(result.into())
        }
    }

    fn new_with_code_and_date(now: DateTime, input_code: String, prefix: &[u8; 3]) -> DBUuid {
        let mut code = String::with_capacity(10 + input_code.len());
        let now = now.0.timestamp_millis() as u64;
        let now_bytes = now.to_be_bytes();
        let mut now_reader = BitReader::endian(Cursor::new(&now_bytes), BigEndian);
        let alphabet = ALPHABET;

        // Generate date chars.
        // Skip first 4 bits to leave only a 60-bit work.
        now_reader.skip(4).unwrap();

        // Skip the first 3 chars to change them by the prefix.
        // This is because the first chars are always --- (0s) due to they are all zeroes.
        // Until 2110, all IDs will start with the prefix, it will vary after that date.
        now_reader.skip(18).unwrap();
        code.push(alphabet[prefix[0] as usize]);
        code.push(alphabet[prefix[1] as usize]);
        code.push(alphabet[prefix[2] as usize]);

        while let Ok(char_index) = now_reader.read::<u8>(6) {
            code.push(alphabet[char_index as usize]);
        }

        // Generate random chars.
        code.push_str(input_code.as_str());

        DBUuid(code.into())
    }

    // GETTERS ----------------------------------------------------------------

    pub fn kind(&self) -> DBUuidType {
        DBUuidType::from_string(&self.0).unwrap()
    }

    pub fn date(&self) -> Option<DateTime> {
        if !self.kind().contains_date() {
            return None;
        }

        let mut milliseconds: u64 = 0;

        // Skip prefix (3 chars) and take next 7.
        let iterator = self.0.chars().skip(3).take(7);

        for char in iterator {
            let index_in_alphabet = *CHAR_SET_REVERSE.get(&char).unwrap();
            milliseconds = (milliseconds << 6) | index_in_alphabet;
        }

        Some(DateTime::new(Utc::timestamp_millis(
            &Utc,
            milliseconds as i64,
        )))
    }

    // METHODS ----------------------------------------------------------------

    pub fn as_string(&self) -> &ArcStr {
        &self.0
    }

    pub fn without_prefix(&self) -> &str {
        &self.0.as_str()[self.kind().prefix_str().len()..]
    }
}

impl Serialize for DBUuid {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for DBUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct UuidVisitor;
        impl<'de> Visitor<'de> for UuidVisitor {
            type Value = DBUuid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with the format of a nanoid")
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
                match check_nanoid(v.as_str(), None) {
                    Ok(_) => Ok(DBUuid(v.into())),
                    Err(e) => Err(E::custom(format!("Incorrect value for a DBUuid: {:?}", e))),
                }
            }
        }

        deserializer.deserialize_string(UuidVisitor)
    }
}

impl FromStr for DBUuid {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_nanoid(s, None).map(|_| DBUuid(s.into()))
    }
}

impl Display for DBUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl DBNormalize for DBUuid {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn check_nanoid(s: &str, kind: Option<DBUuidType>) -> AppResult<()> {
    let guessed_kind = match DBUuidType::from_string(s) {
        Some(v) => v,
        None => {
            return Err(AppError::new(arcstr::literal!(
                "nanoid::decoding::invalid_prefix"
            )));
        }
    };

    if let Some(kind) = kind {
        if kind != guessed_kind {
            return Err(AppError::new(arcstr::literal!(
                "nanoid::decoding::invalid_prefix"
            )));
        }
    }

    if guessed_kind == DBUuidType::Address {
        if Pubkey::from_str(&s[DBUuidType::Address.prefix_str().len()..]).is_err() {
            return Err(AppError::new(arcstr::literal!(
                "nanoid::decoding::invalid_uuid_length"
            )));
        }
    } else if s.len() != guessed_kind.total_chars() {
        return Err(AppError::new(arcstr::literal!(
            "nanoid::decoding::invalid_uuid_length"
        )));
    }

    for c in s.chars() {
        if !CHAR_SET.contains(&c) {
            return Err(AppError::new(arcstr::literal!(
                "nanoid::decoding::invalid_chars"
            )));
        }
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Eq, PartialEq, Hash, EnumList)]
#[repr(u8)]
pub enum DBUuidType {
    DBKey,
    Address,
    APIToken,
    InternalToken,
    Nonce,
}

impl DBUuidType {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_string(text: &str) -> Option<DBUuidType> {
        if text.len() < 3 {
            return None;
        }

        match &text[0..3] {
            x if DBUuidType::DBKey.prefix_str() == x => Some(DBUuidType::DBKey),
            x if DBUuidType::Address.prefix_str() == x => Some(DBUuidType::Address),
            x if DBUuidType::APIToken.prefix_str() == x => Some(DBUuidType::APIToken),
            x if DBUuidType::InternalToken.prefix_str() == x => Some(DBUuidType::InternalToken),
            x if DBUuidType::Nonce.prefix_str() == x => Some(DBUuidType::Nonce),
            _ => None,
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn contains_date(&self) -> bool {
        match self {
            DBUuidType::DBKey => true,
            DBUuidType::Address => false,
            DBUuidType::APIToken => true,
            DBUuidType::InternalToken => false,
            DBUuidType::Nonce => false,
        }
    }

    pub fn random_chars(&self) -> usize {
        match self {
            DBUuidType::DBKey => 15,
            DBUuidType::Address => 44,
            DBUuidType::APIToken => 79,
            DBUuidType::InternalToken => 251,
            DBUuidType::Nonce => 43,
        }
    }

    pub fn total_chars(&self) -> usize {
        const PREFIX_LENGTH: usize = 3;
        let random_chars = self.random_chars();

        if self.contains_date() {
            const DATE_LENGTH: usize = 7;
            PREFIX_LENGTH + DATE_LENGTH + random_chars
        } else {
            PREFIX_LENGTH + random_chars
        }
    }

    pub fn prefix_str(&self) -> &'static str {
        match self {
            DBUuidType::DBKey => "gid",
            DBUuidType::Address => "gad",
            DBUuidType::APIToken => "gat",
            DBUuidType::InternalToken => "git",
            DBUuidType::Nonce => "gno",
        }
    }

    pub fn prefix_char_codes(&self) -> [u8; 3] {
        let prefix_str = self.prefix_str();
        let mut prefix_chars = prefix_str.chars();
        let mut result = [0, 0, 0];

        let char = prefix_chars.next().unwrap();
        result[0] = ALPHABET.iter().position(|v| v == &char).unwrap() as u8;

        let char = prefix_chars.next().unwrap();
        result[1] = ALPHABET.iter().position(|v| v == &char).unwrap() as u8;

        let char = prefix_chars.next().unwrap();
        result[2] = ALPHABET.iter().position(|v| v == &char).unwrap() as u8;

        result
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_types() {
        let now = DateTime::new(Utc.ymd(2021, 8, 15).and_hms_milli(10, 16, 35, 468));

        // DB key.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::DBKey, now.clone());
        let id_ok = DBUuid("gidMh8J1aB000000000000020".into());
        assert_eq!(id, id_ok);

        // Address.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::Address, now.clone());
        let id_ok = DBUuid("gad00000000000000000000000000000000000000000020".into());
        assert_eq!(id, id_ok);

        // API token.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::APIToken, now.clone());
        let id_ok = DBUuid(
            "gatMh8J1aB0000000000000000000000000000000000000000000000000000000000000000000000000000020"
                .into(),
        );
        assert_eq!(id, id_ok);

        // Internal token.
        let id =
            DBUuid::new_with_code_and_date_for_test(20, DBUuidType::InternalToken, now.clone());
        let id_ok = DBUuid(
            "git00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020"
                .into(),
        );
        assert_eq!(id, id_ok);

        // Nonce.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::Nonce, now);
        let id_ok = DBUuid("gno0000000000000000000000000000000000000000020".into());
        assert_eq!(id, id_ok);
    }

    #[test]
    fn test_serialization() {
        let now = DateTime::new(Utc.ymd(2021, 8, 15).and_hms_milli(10, 16, 35, 468));

        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::DBKey, now);
        let serialization = serde_json::to_string(&id).unwrap();

        assert_eq!(serialization, "\"gidMh8J1aB000000000000020\"");

        let deserialization: DBUuid = serde_json::from_str(&serialization).unwrap();
        assert_eq!(deserialization, id);
    }

    #[test]
    fn test_from_str() {
        let id = DBUuid::from_str("gidMh8J1aB000000000000020").expect("The from_str must succeed");
        let id_ok = DBUuid("gidMh8J1aB000000000000020".into());
        assert_eq!(id, id_ok);

        DBUuid::from_str("ImSt").expect_err("The id must fail by prefix");
        DBUuid::from_str("WapImSt").expect_err("The id must fail by length");
        DBUuid::from_str("gidMh8J1aB00000000000002ñ").expect_err("The id must fail by character");
    }

    #[test]
    fn test_date() {
        let now = DateTime::new(Utc.ymd(2021, 8, 15).and_hms_milli(10, 16, 35, 468));

        for kind in DBUuidType::enum_list() {
            let id = DBUuid::new_with_code_and_date_for_test(20, *kind, now.clone());

            if kind.contains_date() {
                assert_eq!(id.date(), Some(now.clone()));
            } else {
                assert_eq!(id.date(), None);
            }
        }
    }

    #[test]
    fn test_prefix() {
        let now = DateTime::new(Utc.ymd(2021, 8, 15).and_hms_milli(10, 16, 35, 468));

        // DB key.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::DBKey, now.clone());
        assert_eq!(id.kind(), DBUuidType::DBKey);

        // Address.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::Address, now.clone());
        assert_eq!(id.kind(), DBUuidType::Address);

        // API token.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::APIToken, now.clone());
        assert_eq!(id.kind(), DBUuidType::APIToken);

        // Internal token.
        let id =
            DBUuid::new_with_code_and_date_for_test(20, DBUuidType::InternalToken, now.clone());
        assert_eq!(id.kind(), DBUuidType::InternalToken);

        // Nonce.
        let id = DBUuid::new_with_code_and_date_for_test(20, DBUuidType::Nonce, now);
        assert_eq!(id.kind(), DBUuidType::Nonce);
    }

    #[test]
    fn test_ordered() {
        let id1 = DBUuid::new(DBUuidType::DBKey);

        // Wait a millisecond.
        sleep(Duration::new(0, 1_000_000));

        let id2 = DBUuid::new(DBUuidType::DBKey);

        // Wait a millisecond.
        sleep(Duration::new(0, 1_000_000));

        let id3 = DBUuid::new(DBUuidType::DBKey);

        assert!(id1.0.as_str() <= id2.0.as_str(), "First");
        assert!(id2.0.as_str() <= id3.0.as_str(), "Second");
    }
}
