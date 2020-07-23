use crate::models::GrandCompany;

use chrono::{DateTime, Utc};
use ffxiv_types::World;
#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct FreeCompanySearchItem {
  #[cfg_attr(feature = "with_serde", serde(with = "crate::util::serde::u64_str"))]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub crest: Vec<Url>,
  pub grand_company: GrandCompany,
  pub active_members: u16,
  pub estate_built: bool,
  pub formed: DateTime<Utc>,
  pub active: Active,
  pub recruitment: RecruitmentStatus,
}

ffxiv_enum!(Active {
  Always => "always",
  Weekdays => "weekdays",
  Weekends => "weekends",
  NotSpecified => "not specified",
});

ffxiv_enum!(RecruitmentStatus {
  Open => "open",
  Closed => "closed",
});
