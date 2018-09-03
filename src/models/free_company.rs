use super::GrandCompany;

use chrono::{DateTime, Utc};

use ffxiv_types::World;

use url::Url;

use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FreeCompany {
  pub id: u64,
  pub name: String,
  pub world: World,
  pub slogan: String,
  #[serde(with = "multi_url")]
  pub crest: Vec<Url>,
  pub grand_company: GrandCompany,
  pub active_members: u16,
  pub rank: u8,
  pub pvp_rankings: PvpRankings,
  pub formed: DateTime<Utc>,
  pub estate: Option<Estate>,
  pub reputation: BTreeMap<GrandCompany, u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpRankings {
  pub weekly: Option<u64>,
  pub monthly: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estate {
  pub name: String,
  pub address: String,
  pub greeting: String,
}

mod multi_url {
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
}
