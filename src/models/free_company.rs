use chrono::{DateTime, Utc};

use ffxiv_types::World;

use url::Url;

#[derive(Debug, Serialize)]
pub struct FreeCompany {
  pub name: String,
  pub world: World,
  pub slogan: String,
  #[serde(serialize_with = "multi_url")]
  pub crest: Vec<Url>,
  pub active_members: u16,
  pub rank: u8,
  pub pvp_rankings: PvpRankings,
  pub formed: DateTime<Utc>,
  pub estate: Option<Estate>,
}

#[derive(Debug, Serialize)]
pub struct PvpRankings {
  pub weekly: Option<u64>,
  pub monthly: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct Estate {
  pub name: String,
  pub address: String,
  pub greeting: String,
}

fn multi_url<S>(urls: &Vec<Url>, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer,
{
  use serde::ser::SerializeSeq;

  let mut seq = serializer.serialize_seq(Some(urls.len()))?;
  for url in urls {
    seq.serialize_element(&url_serde::Ser::new(url))?;
  }
  seq.end()
}
