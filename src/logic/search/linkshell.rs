use crate::{
  error::*,
  logic::plain_parse_elem as plain_parse,
  models::{
    search::{
      Paginated,
      linkshell::LinkshellSearchItem,
    },
  },
};

use ffxiv_types::World;

use scraper::{Html, ElementRef};

use std::str::FromStr;

selectors!(
  ITEM_ENTRY => ".ldst__window > .entry";

  ITEM_ID => ".entry__link--line";
  ITEM_NAME => ".entry__linkshell > .entry__name";
  ITEM_WORLD => ".entry__linkshell > p.entry__world";
  ITEM_ACTIVE_MEMBERS => ".entry .entry__linkshell__member > span";
);

pub fn parse(s: &str) -> Result<Paginated<LinkshellSearchItem>> {
  let html = Html::parse_document(s);

  let pagination = crate::logic::search::parse_pagination(&html)?;

  // has results but requested an invalid page
  if pagination.total_results != 0 && pagination.current_page == 0 {
    return Err(Error::InvalidPage(pagination.total_pages));
  }

  let results: Vec<LinkshellSearchItem> = html
    .select(&*ITEM_ENTRY)
    .map(parse_single)
    .collect::<Result<_>>()?;

  Ok(Paginated {
    pagination,
    results,
  })
}

fn parse_single(html: ElementRef) -> Result<LinkshellSearchItem> {
  let id = parse_id(html)?;
  let name = plain_parse(html, &*ITEM_NAME)?;
  let world = parse_world(html)?;
  let active_members = parse_active_members(html)?;

  Ok(LinkshellSearchItem {
    id,
    name,
    world,
    active_members,
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

fn parse_active_members(html: ElementRef) -> Result<u8> {
  plain_parse(html, &*ITEM_ACTIVE_MEMBERS)
    .and_then(|x| x.parse().map_err(Error::InvalidNumber))
}
