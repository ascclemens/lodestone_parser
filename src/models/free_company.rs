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
  #[serde(with = "crate::util::serde::multi_url")]
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
