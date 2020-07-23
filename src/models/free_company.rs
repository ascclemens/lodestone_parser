use super::GrandCompany;

use chrono::{DateTime, Utc};
use ffxiv_types::World;
#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};
use url::Url;

use std::collections::BTreeMap;

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct FreeCompany {
  #[cfg_attr(feature = "with_serde", serde(with = "crate::util::serde::u64_str"))]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub slogan: String,
  pub crest: Vec<Url>,
  pub grand_company: GrandCompany,
  pub active_members: u16,
  pub rank: u8,
  pub pvp_rankings: PvpRankings,
  pub formed: DateTime<Utc>,
  pub estate: Option<Estate>,
  pub reputation: BTreeMap<GrandCompany, u8>,
}

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct PvpRankings {
  pub weekly: Option<u64>,
  pub monthly: Option<u64>,
}

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct Estate {
  pub name: String,
  pub address: String,
  pub greeting: String,
}
