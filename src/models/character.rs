use super::GrandCompany;

use ffxiv_types::{World, Race, Clan, Guardian};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
  pub id: u64,

  pub name: String,
  pub world: World,
  pub race: Race,
  pub clan: Clan,
  pub gender: Gender,
  pub title: Option<String>,

  pub name_day: String,
  pub guardian: Guardian,
  pub city_state: CityState,

  pub grand_company: Option<GrandCompanyInfo>,
  pub free_company_id: Option<u64>,

  pub profile_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrandCompanyInfo {
  pub grand_company: GrandCompany,
  pub rank: String,
}

ffxiv_enum!(Gender {
  Male => "♂",
  Female => "♀",
});

ffxiv_enum!(CityState {
  Gridania => "Gridania",
  LimsaLominsa => "Limsa Lominsa",
  UlDah => "Ul'dah",
});
