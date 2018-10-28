use super::search::{Paginated, character::CharacterSearchItem};

use ffxiv_types::World;

#[derive(Debug, Serialize, Deserialize)]
pub struct Linkshell {
  #[serde(with = "crate::util::serde::u64_str")]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub active_members: u8,
  pub members: Paginated<LinkshellMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkshellMember {
  #[serde(flatten)]
  pub character: CharacterSearchItem,
  pub role: Option<Role>,
}

ffxiv_enum!(Role {
  Master => "master",
  Leader => "leader",
});
