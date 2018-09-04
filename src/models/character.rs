use super::GrandCompany;

use ffxiv_types::{World, Race, Clan, Guardian};

use url::Url;

use std::collections::BTreeMap;

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

  pub jobs: BTreeMap<Job, JobInfo>,

  #[serde(with = "url_serde")]
  pub face: Url,
  #[serde(with = "url_serde")]
  pub portrait: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrandCompanyInfo {
  pub grand_company: GrandCompany,
  pub rank: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobInfo {
  pub level: Option<u8>,
  pub experience: Option<u64>,
  pub next_level_experience: Option<u64>,
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

ffxiv_enum!(
  #[derive(PartialEq, Eq, PartialOrd, Ord)]
  Job {
    Gladiator => "Gladiator",
    Paladin => "Paladin",
    Marauder => "Marauder",
    Warrior => "Warrior",
    DarkKnight => "Dark Knight",
    Conjurer => "Conjurer",
    WhiteMage => "White Mage",
    Scholar => "Scholar",
    Astrologian => "Astrologian",
    Pugilist => "Pugilist",
    Monk => "Monk",
    Lancer => "Lancer",
    Dragoon => "Dragoon",
    Rogue => "Rogue",
    Ninja => "Ninja",
    Samurai => "Samurai",
    Archer => "Archer",
    Bard => "Bard",
    Machinist => "Machinist",
    Thaumaturge => "Thaumaturge",
    BlackMage => "Black Mage",
    Arcanist => "Arcanist",
    Summoner => "Summoner",
    RedMage => "Red Mage",

    Carpenter => "Carpenter",
    Blacksmith => "Blacksmith",
    Armorer => "Armorer",
    Goldsmith => "Goldsmith",
    Leatherworker => "Leatherworker",
    Weaver => "Weaver",
    Alchemist => "Alchemist",
    Culinarian => "Culinarian",
    Miner => "Miner",
    Botanist => "Botanist",
    Fisher => "Fisher",
  }
);
