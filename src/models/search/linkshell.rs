use ffxiv_types::World;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkshellSearchItem {
  #[serde(with = "crate::util::serde::u64_str")]
  pub id: u64,
  pub name: String,
  pub world: World,
  pub active_members: u8,
}
