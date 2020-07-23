use crate::models::character::GrandCompanyInfo;

use ffxiv_types::World;
#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};

use url::Url;

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct CharacterSearchItem {
  pub id: u64,
  pub name: String,
  pub world: World,
  pub grand_company: Option<GrandCompanyInfo>,
  #[cfg_attr(feature = "with_serde", serde(with = "crate::util::serde::opt_u64_str"))]
  pub free_company_id: Option<u64>,
  pub face: Url,
}
