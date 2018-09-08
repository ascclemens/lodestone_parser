use serde::{Deserializer, Deserialize, Serializer, de::Unexpected};

crate fn serialize<S>(u: &u64, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
{
  serializer.serialize_str(&u.to_string())
}

crate fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
  where D: Deserializer<'de>
{
  let s: String = Deserialize::deserialize(deserializer)?;
  s
    .parse()
    .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(&s), &"string containing a u64"))
}
