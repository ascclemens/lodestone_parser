use crate::{
  error::*,
  logic::plain_parse_elem as plain_parse,
  models::{
    search::{
      Paginated,
      character::CharacterSearchItem,
    },
    character::GrandCompanyInfo,
  }
};

use ffxiv_types::World;

use scraper::{Html, ElementRef};

use url::Url;

use std::str::FromStr;

selectors!(
  ITEM_ENTRY => ".ldst__window .entry";

  ITEM_ID => ".entry__link";
  ITEM_FACE => ".entry__chara__face > img";
  ITEM_NAME => ".entry__name";
  ITEM_WORLD => ".entry__world";
  ITEM_GRAND_COMPANY => ".entry__chara_info .js__tooltip";
  ITEM_FREE_COMPANY => ".entry__freecompany__link";
);

pub fn parse(html: &str) -> Result<Paginated<CharacterSearchItem>> {
  let html = Html::parse_document(html);

  let pagination = crate::logic::search::parse_pagination(&html)?;

  // has results but requested an invalid page
  if pagination.total_results != 0 && pagination.current_page == 0 {
    return Err(Error::InvalidPage(pagination.total_pages));
  }

  let results: Vec<CharacterSearchItem> = html
    .select(&*ITEM_ENTRY)
    .map(parse_single)
    .collect::<Result<_>>()?;

  Ok(Paginated {
    pagination,
    results,
  })
}

crate fn parse_single(html: ElementRef) -> Result<CharacterSearchItem> {
  let id = parse_id(html)?;

  let name = plain_parse(html, &*ITEM_NAME)?;
  let world = parse_world(html)?;

  let grand_company = parse_grand_company(html)?;

  let free_company_id = parse_free_company_id(html)?;

  let face = parse_face(html)?;

  Ok(CharacterSearchItem {
    id,
    name,
    world,
    grand_company,
    free_company_id,
    face,
  })
}

fn parse_id(html: ElementRef) -> Result<u64> {
  let e = html
    .select(&*ITEM_ID)
    .next()
    .ok_or_else(|| Error::missing_element(&*ITEM_ID))?;
  crate::logic::parse_id(e.value())
}

fn parse_world(html: ElementRef) -> Result<World> {
  let world_str = plain_parse(html, &*ITEM_WORLD)?;
  World::from_str(&world_str)
    .map_err(|_| Error::invalid_content("valid world", Some(&world_str)))
}

fn parse_free_company_id(html: ElementRef) -> Result<Option<u64>> {
  let elem = match html
    .select(&*ITEM_FREE_COMPANY)
    .next()
  {
    Some(e) => e,
    None => return Ok(None),
  };
  crate::logic::parse_id(elem.value()).map(Some)
}

fn parse_grand_company(html: ElementRef) -> Result<Option<GrandCompanyInfo>> {
  let text = html
    .select(&*ITEM_GRAND_COMPANY)
    .next()
    .and_then(|x| x.value().attr("data-tooltip"));
  let text = match text {
    Some(t) => t,
    None => return Ok(None),
  };
  crate::logic::parse_grand_company(text).map(Some)
}

fn parse_face(html: ElementRef) -> Result<Url> {
  let face_elem = html
    .select(&*ITEM_FACE)
    .next()
    .ok_or_else(|| Error::missing_element(&*ITEM_FACE))?;
  let src = face_elem
    .value()
    .attr("src")
    .ok_or_else(|| Error::invalid_content("src on face img element", None))?;
  Url::from_str(src).map_err(Error::InvalidUrl)
}
