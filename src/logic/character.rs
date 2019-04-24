use crate::{
  error::*,
  logic::plain_parse,
  models::character::{
    Character,
    CityState,
    Gender,
    GrandCompanyInfo,
    Job,
    JobInfo,
    Minion,
    Mount,
  }
};

use ffxiv_types::{World, Race, Clan, Guardian};

use scraper::{Html, ElementRef};

use url::Url;

use std::{
  collections::BTreeMap,
  str::FromStr,
};

selectors!(
  PROFILE_FACE => ".frame__chara__face > img";
  PROFILE_PORTRAIT => ".character__detail__image > a > img";
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
  PROFILE_MOUNT => ".character__mounts > .character__icon__list .character__item_icon.js__tooltip";
  PROFILE_MINION => ".character__minion > .character__icon__list .character__item_icon.js__tooltip";

  PROFILE_CLASS => "ul.character__job > li";
  CLASS_NAME => ".character__job__name";
  CLASS_LEVEL => ".character__job__level";
  CLASS_EXP => ".character__job__exp";
);

pub fn parse(id: u64, html: &str) -> Result<Character> {
  let html = Html::parse_document(html);

  let name = plain_parse(&html, &*PROFILE_NAME)?;
  let world = parse_world(&html)?;
  let title = parse_title(&html);
  let (race, clan, gender) = parse_rcg(&html)?;

  let name_day = plain_parse(&html, &*PROFILE_NAME_DAY)?;
  let guardian = parse_guardian(&html)?;

  let city_state = parse_city_state(&html)?;

  let grand_company = parse_grand_company(&html)?;

  let free_company_id = parse_free_company_id(&html)?;

  let profile_text = plain_parse(&html, &*PROFILE_TEXT)?.trim().to_string();

  let jobs = parse_jobs(&html)?;

  let mounts = parse_mounts(&html)?;
  let minions = parse_minions(&html)?;

  let face = parse_face(&html)?;
  let portrait = parse_portrait(&html)?;

  Ok(Character {
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
    jobs,
    mounts,
    minions,
    face,
    portrait,
  })
}

fn parse_world(html: &Html) -> Result<World> {
  let parts_str = plain_parse(html, &*PROFILE_WORLD)?;
  let mut parts = parts_str.split("\u{00a0}(");
  let world_str = parts.next()
    .ok_or_else(|| Error::invalid_content("world with data centre in parens", Some(&parts_str)))?;
  World::from_str(world_str)
    .map_err(|_| Error::invalid_content("valid world", Some(&world_str)))
}

fn parse_title(html: &Html) -> Option<String> {
  html
    .select(&*PROFILE_TITLE)
    .next()
    .map(|x| x.text().collect())
}

fn parse_rcg(html: &Html) -> Result<(Race, Clan, Gender)> {
  let mut rcg = html
    .select(&*PROFILE_RACE_CLAN_GENDER)
    .next()
    .ok_or_else(|| Error::missing_element(&*PROFILE_RACE_CLAN_GENDER))?
    .text();

  let race_str = rcg
    .next()
    .ok_or_else(|| Error::invalid_content("first of two parts in race/gender", None))?;
  let race = Race::from_str(race_str)
    .map_err(|_| Error::invalid_content("valid race", Some(race_str)))?;

  let clan_gender_str = rcg
    .next()
    .ok_or_else(|| Error::invalid_content("second of two parts in race/gender", None))?;
  let mut clan_gender_split = clan_gender_str.split(" / ");

  let clan_str = clan_gender_split
    .next()
    .ok_or_else(|| Error::invalid_content("clan/gender split by `/`", Some(clan_gender_str)))?;
  let clan = Clan::from_str(clan_str)
    .map_err(|_| Error::invalid_content("valid clan", Some(clan_str)))?;

  let gender_str = clan_gender_split
    .next()
    .ok_or_else(|| Error::invalid_content("clan/gender split by `/`", Some(clan_gender_str)))?;
  let gender = Gender::parse(gender_str)
    .ok_or_else(|| Error::invalid_content("valid gender", Some(gender_str)))?;

  Ok((race, clan, gender))
}

fn parse_guardian(html: &Html) -> Result<Guardian> {
  let guardian_str = plain_parse(&html, &*PROFILE_GUARDIAN)?;
  guardian_str
    .split(',')
    .next()
    .ok_or_else(|| Error::invalid_content("first part of guardian", Some(&guardian_str)))
    .and_then(|x| Guardian::from_str(&x)
      .map_err(|_| Error::invalid_content("valid guardian", Some(&guardian_str))))
}

fn parse_city_state(html: &Html) -> Result<CityState> {
  let city_state_str = plain_parse(&html, &*PROFILE_CITY_STATE)?;
  CityState::parse(&city_state_str)
    .ok_or_else(|| Error::invalid_content("valid city-state", Some(&city_state_str)))
}

fn parse_grand_company(html: &Html) -> Result<Option<GrandCompanyInfo>> {
  let text = html
    .select(&*PROFILE_GRAND_COMPANY)
    .next()
    .map(|x| x.text().collect::<String>());
  let text = match text {
    Some(t) => t,
    None => return Ok(None),
  };
  crate::logic::parse_grand_company(&text).map(Some)
}

fn parse_free_company_id(html: &Html) -> Result<Option<u64>> {
  let elem = match html
    .select(&*PROFILE_FREE_COMPANY)
    .next()
  {
    Some(e) => e,
    None => return Ok(None),
  };
  crate::logic::parse_id(elem.value()).map(Some)
}

fn parse_face(html: &Html) -> Result<Url> {
  let elem = html
    .select(&*PROFILE_FACE)
    .next()
    .ok_or_else(|| Error::missing_element(&*PROFILE_FACE))?;
  elem
    .value()
    .attr("src")
    .ok_or_else(|| Error::invalid_content("img src attribute", None))
    .and_then(|x| Url::parse(x).map_err(Error::InvalidUrl))
}

fn parse_portrait(html: &Html) -> Result<Url> {
  let elem = html
    .select(&*PROFILE_PORTRAIT)
    .next()
    .ok_or_else(|| Error::missing_element(&*PROFILE_PORTRAIT))?;
  elem
    .value()
    .attr("src")
    .ok_or_else(|| Error::invalid_content("img src attribute", None))
    .and_then(|x| Url::parse(x).map_err(Error::InvalidUrl))
}

fn parse_jobs(html: &Html) -> Result<BTreeMap<Job, JobInfo>> {
  let mut jobs = BTreeMap::new();

  for job in html.select(&*PROFILE_CLASS) {
    let (job, info) = parse_job(job)?;
    jobs.insert(job, info);
  }

  Ok(jobs)
}

fn parse_job(elem: ElementRef) -> Result<(Job, JobInfo)> {
  let job = crate::logic::plain_parse_elem(elem, &*CLASS_NAME)
    .and_then(|x| Job::parse(&x).ok_or_else(|| Error::invalid_content("valid job", Some(&x))))?;

  let level_str = crate::logic::plain_parse_elem(elem, &*CLASS_LEVEL)?;
  let level: Option<u8> = match level_str.as_str() {
    "-" => None,
    x => Some(x.parse().map_err(Error::InvalidNumber)?),
  };

  let exp_str = crate::logic::plain_parse_elem(elem, &*CLASS_EXP)?;
  let mut exp_split = exp_str.split(" / ");

  let first_exp = exp_split.next().unwrap(); // must have first element
  let experience: Option<u64> = match first_exp {
    "-" | "--" => None,
    x => Some(x.replace(",", "").parse().map_err(Error::InvalidNumber)?),
  };

  let second_exp = exp_split
    .next()
    .ok_or_else(|| Error::invalid_content("experience split by ` / `", Some(&exp_str)))?;
  let next_level_experience: Option<u64> = match second_exp {
    "-" | "--" => None,
    x => Some(x.replace(",", "").parse().map_err(Error::InvalidNumber)?),
  };

  let info = JobInfo {
    level,
    experience,
    next_level_experience,
  };

  Ok((job, info))
}

fn parse_minions(html: &Html) -> Result<Vec<Minion>> {
  html.select(&*PROFILE_MINION)
    .map(parse_icon)
    .map(|res| res.map(|(name, icon)| Minion { name, icon }))
    .collect()
}

fn parse_mounts(html: &Html) -> Result<Vec<Mount>> {
  html.select(&*PROFILE_MOUNT)
    .map(parse_icon)
    .map(|res| res.map(|(name, icon)| Mount { name, icon }))
    .collect()
}

fn parse_icon(elem: ElementRef) -> Result<(String, Url)> {
  let name = elem
    .value()
    .attr("data-tooltip")
    .ok_or_else(|| Error::invalid_content("data-tooltip on icon", None))?
    .to_string();
  let image = elem
    .children()
    .flat_map(|c| c.value().as_element())
    .find(|c| c.name() == "img")
    .ok_or_else(|| Error::invalid_content("img in icon", None))?
    .attr("src")
    .ok_or_else(|| Error::invalid_content("img src in icon", None))?;

  let icon = Url::from_str(image).map_err(Error::InvalidUrl)?;

  Ok((name, icon))
}
