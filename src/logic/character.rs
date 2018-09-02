use crate::models::character::{
  Character,
  CityState,
  Gender,
  GrandCompany,
  GrandCompanyInfo,
};

use ffxiv_types::{Race, Clan, Guardian};

use scraper::Html;

use std::str::FromStr;

selectors!(
  PROFILE_NAME => ".frame__chara__name";
  PROFILE_WORLD => ".frame__chara__world";
  PROFILE_TITLE => ".frame__chara__title";
  PROFILE_NAME_DAY => ".character-block__birth";
  PROFILE_RACE_CLAN_GENDER => "div.character-block:nth-of-type(1) > .character-block__box > .character-block__name";
  PROFILE_GUARDIAN => "div.character-block:nth-of-type(2) > .character-block__box > .character-block__name";
  PROFILE_CITY_STATE => "div.character-block:nth-of-type(3) > .character-block__box > .character-block__name";
  PROFILE_GRAND_COMPANY => "div.character-block:nth-of-type(4) > .character-block__box > .character-block__name";
  PROFILE_FREE_COMPANY => ".character__freecompany__name > h4 > a";
  PROFILE_TEXT => ".character__selfintroduction";
);

pub fn parse(id: u64, html: &str) -> Option<Character> {
  let html = Html::parse_document(html);

  let name = html.select(&*PROFILE_NAME).next()?.text().collect();

  let world_str: String = html.select(&*PROFILE_WORLD).next()?.text().collect();
  let world = ffxiv_types::World::from_str(&world_str).ok()?;

  let title: Option<String> = html
    .select(&*PROFILE_TITLE)
    .next()
    .map(|x| x.text().collect());

  let mut rcg = html.select(&*PROFILE_RACE_CLAN_GENDER).next()?.text();

  let race = Race::from_str(rcg.next()?).ok()?;

  let mut clan_gender_str = rcg.next()?.split(" / ");

  let clan = Clan::from_str(clan_gender_str.next()?).ok()?;

  let gender = Gender::parse(clan_gender_str.next()?)?;

  let name_day = html.select(&*PROFILE_NAME_DAY).next()?.text().collect();

  let guardian_str: String = html.select(&*PROFILE_GUARDIAN).next()?.text().collect();
  let guardian = Guardian::from_str(guardian_str.split(",").next()?).ok()?;

  let city_state_str: String = html.select(&*PROFILE_CITY_STATE).next()?.text().collect();
  let city_state = CityState::parse(&city_state_str)?;

  let grand_company: Option<GrandCompanyInfo> = html
    .select(&*PROFILE_GRAND_COMPANY)
    .next()
    .map(|x| x.text().collect::<String>())
    .and_then(|x| {
      let mut x = x.split(" / ");
      let gc = GrandCompany::parse(x.next()?)?;
      Some(GrandCompanyInfo {
        grand_company: gc,
        rank: x.next()?.to_string(),
      })
    });

  let free_company_id: Option<u64> = html
    .select(&*PROFILE_FREE_COMPANY)
    .next()
    .and_then(|x| x.value().attr("href"))
    .and_then(|x| x
      .split('/')
      .filter(|x| !x.is_empty())
      .last())
    .and_then(|x| x.parse().ok());

  let profile_text = html
    .select(&*PROFILE_TEXT)
    .next()?
    .text()
    .collect::<String>()
    .trim()
    .to_string();

  Some(Character {
    id,
    name,
    world,
    race,
    clan,
    gender,
    title,
    name_day,
    guardian,
    city_state,
    grand_company,
    free_company_id,
    profile_text,
  })
}
