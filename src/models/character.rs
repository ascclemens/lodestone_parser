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
  #[serde(with = "crate::util::serde::opt_u64_str")]
  pub free_company_id: Option<u64>,

  pub profile_text: String,

  pub jobs: BTreeMap<Job, JobInfo>,

  #[serde(with = "url_serde")]
  pub face: Url,
  #[serde(with = "url_serde")]
  pub portrait: Url,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GrandCompanyInfo {
  pub name: GrandCompany,
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
  Gridania => "gridania",
  LimsaLominsa => "limsa lominsa",
  UlDah => "ul'dah",
});

ffxiv_enum!(
  #[derive(Eq, PartialOrd, Ord)]
  Job {
    Gladiator => "gladiator",
    Paladin => "paladin",
    Marauder => "marauder",
    Warrior => "warrior",
    DarkKnight => "dark knight",
    Conjurer => "conjurer",
    WhiteMage => "white mage",
    Scholar => "scholar",
    Astrologian => "astrologian",
    Pugilist => "pugilist",
    Monk => "monk",
    Lancer => "lancer",
    Dragoon => "dragoon",
    Rogue => "rogue",
    Ninja => "ninja",
    Samurai => "samurai",
    Archer => "archer",
    Bard => "bard",
    Machinist => "machinist",
    Thaumaturge => "thaumaturge",
    BlackMage => "black mage",
    Arcanist => "arcanist",
    Summoner => "summoner",
    RedMage => "red mage",
    BlueMage => "blue mage",

    Carpenter => "carpenter",
    Blacksmith => "blacksmith",
    Armorer => "armorer",
    Goldsmith => "goldsmith",
    Leatherworker => "leatherworker",
    Weaver => "weaver",
    Alchemist => "alchemist",
    Culinarian => "culinarian",
    Miner => "miner",
    Botanist => "botanist",
    Fisher => "fisher",
  }
);
