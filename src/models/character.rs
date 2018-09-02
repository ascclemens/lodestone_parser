use ffxiv_types::World;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct GrandCompanyInfo {
  pub grand_company: GrandCompany,
  pub rank: String,
}

ffxiv_enum!(Gender {
  Male => "♂",
  Female => "♀",
});

ffxiv_enum!(Race {
  AuRa => "Au Ra",
  Elezen => "Elezen",
  Hyur => "Hyur",
  Lalafell => "Lalafell",
  Miqote => "Miqo'te",
  Roegadyn => "Roegadyn",
});

ffxiv_enum!(Clan {
  Raen => "Raen",
  Xaela => "Xaela",
  Duskwight => "Duskwight",
  Wildwood => "Wildwood",
  Highlander => "Highlander",
  Midlander => "Midlander",
  Dunesfolk => "Dunesfolk",
  Plainsfolk => "Plainsfolk",
  SeekerOfTheMoon => "Seeker of the Moon",
  SeekerOfTheSun => "Seeker of the Sun",
  Hellsguard => "Hellsguard",
  SeaWolf => "Sea Wolf",
});

ffxiv_enum!(GrandCompany {
  Flames => "Immortal Flames",
  Maelstrom => "Maelstrom",
  TwinAdders => "Order of the Twin Adder",
});

ffxiv_enum!(Guardian {
  Althyk => "Althyk, the Keeper",
  Halone => "Halone, the Fury",
  Menphina => "Menphina, the Lover",
  Oschon => "Oschon, the Wanderer",
  Rhalgr => "Rhalgr, the Destroyer",
});

ffxiv_enum!(CityState {
  Gridania => "Gridania",
  LimsaLominsa => "Limsa Lominsa",
  UlDah => "Ul'dah",
});
