use crate::{
  error::*,
  logic::plain_parse,
  models::{
    linkshell::{Linkshell, LinkshellMember, Role},
    search::Paginated,
  },
};

use ffxiv_types::World;

use scraper::{Html, ElementRef};

use std::str::FromStr;

selectors!(
  LS_NAME => "h3.heading__linkshell__name";
  LS_WORLD => "p.entry__world";
  LS_ACTIVE_MEMBERS => "div.parts__total";

  ITEM_ENTRY => ".ldst__window .entry";
  ITEM_ROLE => ".entry__chara_info__linkshell > span";
);

pub fn parse(id: u64, html_str: &str) -> Result<Linkshell> {
  let html = Html::parse_document(html_str);

  let name = plain_parse(&html, &*LS_NAME)?;
  let world = parse_world(&html)?;
  let active_members = parse_active_members(&html)?;
  let members = parse_members(html_str)?;

  Ok(Linkshell {
    id,
    name,
    world,
    active_members,
    members,
  })
}

fn parse_world(html: &Html) -> Result<World> {
  let world_str = plain_parse(html, &*LS_WORLD)?;
  let trimmed = world_str.trim();
  World::from_str(trimmed)
    .map_err(|_| Error::invalid_content("a world", Some(trimmed)))
}

fn parse_active_members(html: &Html) -> Result<u8> {
  let text = plain_parse(&html, &*LS_ACTIVE_MEMBERS)?;
  text.split(' ').next().unwrap().parse().map_err(Error::InvalidNumber)
}

fn parse_members(html_str: &str) -> Result<Paginated<LinkshellMember>> {
  let html = Html::parse_document(html_str);

  let pagination = crate::logic::search::parse_pagination(&html)?;

  // has results but requested an invalid page
  if pagination.total_results != 0 && pagination.current_page == 0 {
    return Err(Error::InvalidPage(pagination.total_pages));
  }

  let results: Vec<LinkshellMember> = html
    .select(&*ITEM_ENTRY)
    .map(|x| parse_single(x))
    .collect::<Result<_>>()?;

  Ok(Paginated {
    pagination,
    results,
  })
}


fn parse_single<'a>(html: ElementRef<'a>) -> Result<LinkshellMember> {
  let character = super::search::character::parse_single(html)?;

  let role = parse_role(html)?;

  Ok(LinkshellMember {
    character,
    role,
  })
}

fn parse_role<'a>(html: ElementRef<'a>) -> Result<Option<Role>> {
  let role = match html.select(&*ITEM_ROLE).next() {
    Some(r) => r,
    None => return Ok(None),
  };
  let role_str: String = role.text().collect();
  Role::parse(role_str.trim())
    .ok_or_else(|| Error::invalid_content("valid linkshell role", Some(role_str.trim())))
    .map(Some)
}
