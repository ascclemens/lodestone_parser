use crate::{
  error::*,
  logic::plain_parse_elem as plain_parse,
  models::{
    GrandCompany,
    search::{
      Paginated,
      free_company::{FreeCompanySearchItem, Active, RecruitmentStatus},
    },
  },
};

use chrono::{DateTime, Local, TimeZone, Utc};

use ffxiv_types::World;

use scraper::{Html, ElementRef};

use url::Url;

use std::str::FromStr;

selectors!(
  ITEM_ENTRY => ".ldst__window > .entry";

  ITEM_ID => ".entry__block";
  ITEM_GRAND_COMPANY => ".entry__freecompany__box > p.entry__world:nth-of-type(1)";
  ITEM_NAME => ".entry__freecompany__box > .entry__name";
  ITEM_WORLD => ".entry__freecompany__box > p.entry__world:nth-of-type(3)";
  ITEM_CREST => ".entry__freecompany__crest__image > img";
  ITEM_ACTIVE_MEMBERS => ".entry__freecompany__fc-member";
  ITEM_ESTATE_BUILT => ".entry__freecompany__fc-housing";
  ITEM_FORMED => ".entry__freecompany__fc-day > script";
  ITEM_ACTIVE => ".entry__freecompany__fc-data > li.entry__freecompany__fc-active:nth-of-type(4)";
  ITEM_RECRUITMENT => ".entry__freecompany__fc-data > li.entry__freecompany__fc-active:nth-of-type(5)";
);

pub fn parse(s: &str) -> Result<Paginated<FreeCompanySearchItem>> {
  let html = Html::parse_document(s);

  let pagination = crate::logic::search::parse_pagination(&html)?;

  let results: Vec<FreeCompanySearchItem> = html
    .select(&*ITEM_ENTRY)
    .map(|x| parse_single(x))
    .collect::<Result<_>>()?;

  Ok(Paginated {
    pagination,
    results,
  })
}

fn parse_single<'a>(html: ElementRef<'a>) -> Result<FreeCompanySearchItem> {
  let id = parse_id(html)?;
  let grand_company = parse_grand_company(html)?;
  let name = plain_parse(html, &*ITEM_NAME)?;
  let world = parse_world(html)?;
  let crest = parse_crest(html)?;
  let active_members = parse_active_members(html)?;
  let estate_built = parse_estate_built(html)?;
  let formed = parse_formed(html)?;
  let active = parse_active(html)?;
  let recruitment = parse_recruitment(html)?;

  Ok(FreeCompanySearchItem {
    id,
    grand_company,
    name,
    world,
    crest,
    active_members,
    estate_built,
    formed,
    active,
    recruitment,
  })
}

fn parse_id<'a>(html: ElementRef<'a>) -> Result<u64> {
  let e = html
    .select(&*ITEM_ID)
    .next()
    .ok_or_else(|| Error::missing_element(&*ITEM_ID))?;
  crate::logic::parse_id(e.value())
}

fn parse_grand_company<'a>(html: ElementRef<'a>) -> Result<GrandCompany> {
  let gc_str = plain_parse(html, &*ITEM_GRAND_COMPANY)?;
  GrandCompany::parse(&gc_str)
    .ok_or_else(|| Error::invalid_content("valid grand company", Some(&gc_str)))
}

fn parse_world<'a>(html: ElementRef<'a>) -> Result<World> {
  let world_str = plain_parse(html, &*ITEM_WORLD)?;
  World::from_str(&world_str)
    .map_err(|_| Error::invalid_content("valid world", Some(&world_str)))
}

fn parse_crest<'a>(html: ElementRef<'a>) -> Result<Vec<Url>> {
  html.select(&*ITEM_CREST)
    .filter_map(|x| x.value().attr("src"))
    .map(|x| Url::from_str(x).map_err(Error::InvalidUrl))
    .collect()
}

fn parse_active_members<'a>(html: ElementRef<'a>) -> Result<u16> {
  plain_parse(html, &*ITEM_ACTIVE_MEMBERS)
    .and_then(|x| x.parse().map_err(Error::InvalidNumber))
}

fn parse_estate_built<'a>(html: ElementRef<'a>) -> Result<bool> {
  let estate_built = plain_parse(html, &*ITEM_ESTATE_BUILT)?;
  let built = match estate_built.as_str() {
    "Estate Built" => true,
    "No Estate or Plot" => false,
    _ => return Err(Error::invalid_content("`Estate Built` or `No Estate or Plot`", Some(&estate_built))),
  };
  Ok(built)
}

fn parse_formed<'a>(html: ElementRef<'a>) -> Result<DateTime<Utc>> {
  let script = html
    .select(&*ITEM_FORMED)
    .next()
    .ok_or(Error::missing_element(&*ITEM_FORMED))?
    .inner_html();

  let timestamp = script
    .split("strftime(")
    .nth(1)
    .ok_or(Error::invalid_content("strftime call", Some(&script)))?
    .split(",")
    .next()
    .ok_or(Error::invalid_content("comma-separated strftime call", Some(&script)))?;
  let timestamp: i64 = timestamp.parse().map_err(Error::InvalidNumber)?;

  let utc = Local.timestamp(timestamp, 0).with_timezone(&Utc);

  Ok(utc)
}

fn parse_active<'a>(html: ElementRef<'a>) -> Result<Active> {
  plain_parse(html, &*ITEM_ACTIVE)
    .and_then(|x| x
      .split(": ")
      .nth(1)
      .map(ToString::to_string)
      .ok_or_else(|| Error::invalid_content("activity split by `: `", Some(&x))))
    .and_then(|x| Active::parse(&x)
      .ok_or_else(|| Error::invalid_content("valid activity", Some(&x))))
}

fn parse_recruitment<'a>(html: ElementRef<'a>) -> Result<RecruitmentStatus> {
  plain_parse(html, &*ITEM_RECRUITMENT)
    .and_then(|x| x
      .split(": ")
      .nth(1)
      .map(ToString::to_string)
      .ok_or_else(|| Error::invalid_content("recruitment status split by `: `", Some(&x))))
    .and_then(|x| RecruitmentStatus::parse(&x)
      .ok_or_else(|| Error::invalid_content("valid recruitment status", Some(&x))))
}
