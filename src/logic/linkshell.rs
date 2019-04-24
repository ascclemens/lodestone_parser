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
  let parts_str = plain_parse(html, &*LS_WORLD)?;
  let mut parts = parts_str.split("\u{00a0}(");
  let world_str = parts.next()
    .ok_or_else(|| Error::invalid_content("world with data centre in parens", Some(&parts_str)))?;
  World::from_str(world_str)
    .map_err(|_| Error::invalid_content("valid world", Some(&world_str)))
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
    .map(parse_single)
    .collect::<Result<_>>()?;

  Ok(Paginated {
    pagination,
    results,
  })
}

fn parse_single(html: ElementRef) -> Result<LinkshellMember> {
  let character = super::search::character::parse_single(html)?;

  let role = parse_role(html)?;

  Ok(LinkshellMember {
    character,
    role,
  })
}

fn parse_role(html: ElementRef) -> Result<Option<Role>> {
  let role = match html.select(&*ITEM_ROLE).next() {
    Some(r) => r,
    None => return Ok(None),
  };
  let role_str: String = role.text().collect();
  Role::parse(role_str.trim())
    .ok_or_else(|| Error::invalid_content("valid linkshell role", Some(role_str.trim())))
    .map(Some)
}

#[cfg(test)]
mod test {
  use crate::models::{
    GrandCompany,
    character::GrandCompanyInfo,
    linkshell::{Linkshell, Role}
  };
  use super::parse;

  use lazy_static::lazy_static;
  use ffxiv_types::World;

  lazy_static! {
    static ref PARSED: crate::error::Result<Linkshell> = {
      let html = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/html/Linkshells/lala world.html"));
      parse(20547673299957974, &html)
    };
  }

  #[test]
  fn is_ok() {
    assert!(PARSED.is_ok());
  }

  #[test]
  fn metadata() {
    let ls = PARSED.as_ref().unwrap();
    assert_eq!(20547673299957974, ls.id);
    assert_eq!("lala world", ls.name);
    assert_eq!(World::Adamantoise, ls.world);
    assert_eq!(104, ls.active_members);
    assert_eq!(50, ls.members.results.len());
    assert_eq!(1, ls.members.pagination.current_page);
    assert_eq!(3, ls.members.pagination.total_pages);
    assert_eq!(104, ls.members.pagination.total_results);
  }

  #[test]
  fn prinny() {
    let prinny = &PARSED.as_ref().unwrap().members.results[0];
    assert_eq!(6500087, prinny.id);
    assert_eq!("Prinny Dawnbringer", prinny.name);
    assert_eq!(World::Adamantoise, prinny.world);
    assert_eq!(
      Some(GrandCompanyInfo {
        name: GrandCompany::Maelstrom,
        rank: "Storm Captain".into(),
      }),
      prinny.grand_company,
    );
    assert_eq!(Some(9233645873504743773), prinny.free_company_id);
    assert_eq!(
      "https://img2.finalfantasyxiv.com/f/8089bddc032754e155ff2f75925c8c26_1f5fd239b885860b7c2bfc72ad1d97effc0_96x96.jpg?1556144778",
      prinny.face.as_str(),
    );
    assert_eq!(Some(Role::Master), prinny.role);
  }
}
