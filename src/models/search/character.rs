use crate::models::character::GrandCompanyInfo;

use ffxiv_types::World;

use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSearchItem {
  pub id: u64,
  pub name: String,
  pub world: World,
  pub grand_company: Option<GrandCompanyInfo>,
  pub free_company_id: Option<u64>,
  #[serde(with = "url_serde")]
  pub face: Url,
}
