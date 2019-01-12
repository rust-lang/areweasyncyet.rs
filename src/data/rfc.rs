use super::IssueId;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Rfc {
    id: IssueId,
    url: String,
    merged: bool,
}

impl<'de> Deserialize<'de> for Rfc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RfcVisitor;
        impl<'de> Visitor<'de> for RfcVisitor {
            type Value = Rfc;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("rfc number or page name")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let id = value as IssueId;
                let url = format!("https://github.com/rust-lang/rfcs/pull/{}", value);
                Ok(Rfc {
                    id,
                    url,
                    merged: false,
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let id: IssueId = value
                    .find('-')
                    .and_then(|dash| value[..dash].parse().ok())
                    .ok_or_else(|| E::custom(format!("invalid page name: {}", value)))?;
                let hash = value.find('#').unwrap_or(value.len());
                let (page, frag) = value.split_at(hash);
                let url = format!("https://rust-lang.github.io/rfcs/{}.html{}", page, frag);
                Ok(Rfc {
                    id,
                    url,
                    merged: true,
                })
            }
        }
        deserializer.deserialize_u64(RfcVisitor)
    }
}
