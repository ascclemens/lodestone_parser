use serde::{Deserializer, Deserialize, Serializer, ser::SerializeSeq};

use url::Url;

crate fn serialize<S>(urls: &Vec<Url>, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
{
  let mut seq = serializer.serialize_seq(Some(urls.len()))?;
  for url in urls {
    seq.serialize_element(&url_serde::Ser::new(url))?;
  }
  seq.end()
}

crate fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Url>, D::Error>
  where D: Deserializer<'de>
{

  #[derive(Deserialize)]
  struct Wrapper(#[serde(with = "url_serde")] Url);

  let urls = Vec::deserialize(deserializer)?;
  Ok(urls.into_iter().map(|Wrapper(u)| u).collect())
}
