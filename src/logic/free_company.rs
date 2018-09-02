use crate::{
  error::*,
  models::free_company::{FreeCompany, PvpRankings, Estate},
};

use chrono::{DateTime, Local, TimeZone, Utc};

use ffxiv_types::World;

use scraper::Html;

use url::Url;

use std::str::FromStr;

selectors!(
  FC_NAME => ".entry__freecompany__name";
  FC_WORLD => "p.entry__freecompany__gc:nth-of-type(3)";
  FC_SLOGAN => ".freecompany__text__message.freecompany__text";
  FC_TAG => ".freecompany__text__tag.freecompany__text";
  FC_CREST => ".entry__freecompany__crest__image > img";
  FC_FORMED => "p.freecompany__text:nth-of-type(5) > script";
  FC_ACTIVE_MEMBERS => "p.freecompany__text:nth-of-type(6)";
  FC_RANK => "p.freecompany__text:nth-of-type(7)";
  FC_WEEKLY_RANKING => ".character__ranking__data tr:nth-of-type(1) > th";
  FC_MONTHLY_RANKING => ".character__ranking__data tr:nth-of-type(2) > th";

  FC_ESTATE_MISSING => ".freecompany__estate__none";
  FC_ESTATE_NAME => ".freecompany__estate__name";
  FC_ESTATE_ADDRESS => ".freecompany__estate__text";
  FC_ESTATE_GREETING => ".freecompany__estate__greeting";
);

pub fn parse(id: u64, html: &str) -> Result<FreeCompany> {
  let html = Html::parse_document(html);

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

  Ok(FreeCompany {
    name,
    world,
    slogan,
    crest,
    active_members,
    rank,
    pvp_rankings,
    formed,
    estate,
  })
}

fn plain_parse(html: &Html, select: &scraper::Selector) -> Result<String> {
  let string = html
    .select(select)
    .next()
    .ok_or(Error::missing_element(select))?
    .text()
    .collect();
  Ok(string)
}

fn parse_world(html: &Html) -> Result<World> {
  let world_str = plain_parse(html, &*FC_WORLD)?;
  let trimmed = world_str.trim();
  World::from_str(trimmed)
    .map_err(|_| Error::invalid_content("a world", Some(trimmed)))
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
    .split(":")
    .nth(1)
    .ok_or_else(|| Error::invalid_content("colon-separated text", Some(&rank_str)))
    .and_then(|x| x
      .split(" ")
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
    .ok_or(Error::missing_element(&*FC_FORMED))?
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
    .into_iter()
    .filter_map(|x| x.value().attr("src"))
    .map(|x| Url::parse(x).map_err(Error::InvalidUrl))
    .collect()
}
