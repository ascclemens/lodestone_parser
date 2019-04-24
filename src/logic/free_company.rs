use crate::{
  error::*,
  logic::plain_parse,
  models::GrandCompany,
  models::free_company::{FreeCompany, PvpRankings, Estate},
};

use chrono::{DateTime, Local, TimeZone, Utc};

use ffxiv_types::World;

use scraper::Html;

use url::Url;

use std::{
  collections::BTreeMap,
  str::FromStr,
};

selectors!(
  FC_GRAND_COMPANY => "p.entry__freecompany__gc:nth-of-type(1)";
  FC_NAME => ".entry__freecompany__name";
  FC_WORLD => "p.entry__freecompany__gc:nth-of-type(3)";
  FC_SLOGAN => ".freecompany__text__message.freecompany__text";
  FC_TAG => ".freecompany__text__tag.freecompany__text";
  FC_CREST => ".entry__freecompany__crest__image > img";
  FC_FORMED => "p.freecompany__text:nth-of-type(5) > script";
  FC_ACTIVE_MEMBERS => "p.freecompany__text:nth-of-type(6)";
  FC_RANK => "p.freecompany__text:nth-of-type(7)";

  FC_REPUTATION => ".freecompany__reputation__data";
  FC_REPUTATION_NAME => ".freecompany__reputation__gcname";
  FC_REPUTATION_RANK => ".freecompany__reputation__rank";

  FC_WEEKLY_RANKING => ".character__ranking__data tr:nth-of-type(1) > th";
  FC_MONTHLY_RANKING => ".character__ranking__data tr:nth-of-type(2) > th";

  FC_ESTATE_MISSING => ".freecompany__estate__none";
  FC_ESTATE_NAME => ".freecompany__estate__name";
  FC_ESTATE_ADDRESS => ".freecompany__estate__text";
  FC_ESTATE_GREETING => ".freecompany__estate__greeting";
);

pub fn parse(id: u64, html: &str) -> Result<FreeCompany> {
  let html = Html::parse_document(html);

  let grand_company = parse_grand_company(&html)?;
  let name = plain_parse(&html, &*FC_NAME)?;
  let world = parse_world(&html)?;
  let slogan = plain_parse(&html, &*FC_SLOGAN)?;
  let crest = parse_crest(&html)?;
  let active_members = parse_active_members(&html)?;
  let rank = parse_rank(&html)?;
  let pvp_rankings = PvpRankings {
    weekly: parse_pvp_rank(&html, &*FC_WEEKLY_RANKING)?,
    monthly: parse_pvp_rank(&html, &*FC_MONTHLY_RANKING)?,
  };
  let formed = parse_formed(&html)?;
  let estate = parse_estate(&html)?;
  let reputation = parse_reputation(&html)?;

  Ok(FreeCompany {
    id,
    name,
    world,
    slogan,
    crest,
    grand_company,
    active_members,
    rank,
    pvp_rankings,
    formed,
    estate,
    reputation,
  })
}

fn parse_world(html: &Html) -> Result<World> {
  let parts_str = plain_parse(html, &*FC_WORLD)?;
  let mut parts = parts_str.split("\u{00a0}(");
  let world_str = parts.next()
    .ok_or_else(|| Error::invalid_content("world with data centre in parens", Some(&parts_str)))?;
  World::from_str(world_str.trim())
    .map_err(|_| Error::invalid_content("valid world", Some(&world_str)))
}

fn parse_active_members(html: &Html) -> Result<u16> {
  plain_parse(&html, &*FC_ACTIVE_MEMBERS)
    .and_then(|x| x.parse().map_err(Error::InvalidNumber))
}

fn parse_rank(html: &Html) -> Result<u8> {
  plain_parse(&html, &*FC_RANK)
    .and_then(|x| x.parse().map_err(Error::InvalidNumber))
}

fn parse_pvp_rank(html: &Html, select: &scraper::Selector) -> Result<Option<u64>> {
  let rank_str = plain_parse(html, select)?;

  let rank = rank_str
    .split(':')
    .nth(1)
    .ok_or_else(|| Error::invalid_content("colon-separated text", Some(&rank_str)))
    .and_then(|x| x
      .split(' ')
      .next()
      .ok_or_else(|| Error::invalid_content("space-separated text", Some(&rank_str))))?;

  if rank == "--" {
    return Ok(None);
  }

  rank
    .parse()
    .map(Some)
    .map_err(Error::InvalidNumber)
}

fn parse_formed(html: &Html) -> Result<DateTime<Utc>> {
  let script = html
    .select(&*FC_FORMED)
    .next()
    .ok_or_else(|| Error::missing_element(&*FC_FORMED))?
    .inner_html();

  let timestamp = script
    .split("strftime(")
    .nth(1)
    .ok_or_else(|| Error::invalid_content("strftime call", Some(&script)))?
    .split(',')
    .next()
    .ok_or_else(|| Error::invalid_content("comma-separated strftime call", Some(&script)))?;
  let timestamp: i64 = timestamp.parse().map_err(Error::InvalidNumber)?;

  let utc = Local.timestamp(timestamp, 0).with_timezone(&Utc);

  Ok(utc)
}

fn parse_estate(html: &Html) -> Result<Option<Estate>> {
  if html.select(&*FC_ESTATE_MISSING).next().is_some() {
    return Ok(None);
  }

  let name = plain_parse(html, &*FC_ESTATE_NAME)?;
  let address = plain_parse(html, &*FC_ESTATE_ADDRESS)?;
  let greeting = plain_parse(html, &*FC_ESTATE_GREETING)?;

  Ok(Some(Estate {
    name,
    address,
    greeting,
  }))
}

fn parse_crest(html: &Html) -> Result<Vec<Url>> {
  html.select(&*FC_CREST)
    .filter_map(|x| x.value().attr("src"))
    .map(|x| Url::parse(x).map_err(Error::InvalidUrl))
    .collect()
}

fn parse_grand_company(html: &Html) -> Result<GrandCompany> {
  let text = plain_parse(html, &*FC_GRAND_COMPANY)?;
  let name = text
    .split(" <")
    .next()
    .ok_or_else(|| Error::invalid_content("grand company and reputation", Some(&text)))?;
  GrandCompany::parse(&name)
    .ok_or_else(|| Error::invalid_content("valid grand company", Some(&name)))
}

fn parse_reputation(html: &Html) -> Result<BTreeMap<GrandCompany, u8>> {
  let mut reps = BTreeMap::new();

  for elem in html.select(&*FC_REPUTATION) {
    let name: String = elem
      .select(&*FC_REPUTATION_NAME)
      .next()
      .ok_or_else(|| Error::missing_element(&*FC_REPUTATION_NAME))?
      .text()
      .collect();
    let gc = GrandCompany::parse(&name)
      .ok_or_else(|| Error::invalid_content("valid grand company", Some(&name)))?;
    let rank_elem = elem
      .select(&*FC_REPUTATION_RANK)
      .next()
      .ok_or_else(|| Error::missing_element(&*FC_REPUTATION_RANK))?;
    let color_class = rank_elem
      .value()
      .classes()
      .find(|x| x.starts_with("color_"))
      .ok_or_else(|| Error::invalid_content("color_## class", None))?;
    let rank: u8 = color_class
      .split("color_")
      .nth(1)
      .ok_or_else(|| Error::invalid_content("color_## class", Some(&color_class)))
      .and_then(|x| x.parse().map_err(Error::InvalidNumber))?;
    reps.insert(gc, rank);
  }

  if reps.len() != 3 {
    return Err(Error::invalid_content("three grand companies with reputation", None));
  }

  Ok(reps)
}
