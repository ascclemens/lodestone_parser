use ffxiv_types::World;
#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct LinkshellSearchItem {
  #[cfg_attr(feature = "with_serde", serde(with = "crate::util::serde::u64_str"))]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub active_members: u8,
}
