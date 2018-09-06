use serde::{Deserializer, Deserialize, Serializer, de::Unexpected};

crate fn serialize<S>(u: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
{
  match *u {
    Some(u) => serializer.serialize_str(&u.to_string()),
    None => serializer.serialize_none(),
  }
}

crate fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
  where D: Deserializer<'de>
{
  let s: Option<&str> = Deserialize::deserialize(deserializer)?;
  let s = match s {
    Some(s) => s,
    None => return Ok(None),
  };
  s
    .parse()
    .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(s), &"string containing a u64"))
    .map(Some)
}
