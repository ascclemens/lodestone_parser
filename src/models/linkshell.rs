use super::search::{Paginated, character::CharacterSearchItem};

use ffxiv_types::World;
#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct Linkshell {
  #[cfg_attr(feature = "with_serde", serde(with = "crate::util::serde::u64_str"))]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub active_members: u8,
  pub members: Paginated<LinkshellMember>,
}

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct LinkshellMember {
  #[cfg_attr(feature = "with_serde", serde(flatten))]
  pub character: CharacterSearchItem,
  pub role: Option<Role>,
}

impl std::ops::Deref for LinkshellMember {
  type Target = CharacterSearchItem;

  fn deref(&self) -> &Self::Target {
    &self.character
  }
}

ffxiv_enum!(Role {
  Master => "master",
  Leader => "leader",
});
