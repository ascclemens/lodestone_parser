use crate::{
  error::*,
  models::search::Pagination,
};

use scraper::Html;

pub mod character;
pub mod free_company;

pub use self::{
  character::parse as parse_character_search,
  free_company::parse as parse_free_company_search,
};

selectors!(
  PAGINATION_TOTAL => ".parts__total";
  PAGINATION_PAGES => ".btn__pager__current";

  NO_RESULTS => "p.parts__zero";
);

crate fn parse_pagination(html: &Html) -> Result<Pagination> {
  const LODESTONE_PER_PAGE: f32 = 50.0;

  let total_str = crate::logic::plain_parse(&html, &*PAGINATION_TOTAL)?;
  let total_results: u64 = total_str
    .split(' ')
    .next()
    .unwrap() // will always have a first element
    .parse()
    .map_err(Error::InvalidNumber)?;

  let (total_pages, current_page) = if parse_no_results(html) {
    let total_pages = if total_results > 0 {
      (total_results as f32 / LODESTONE_PER_PAGE).ceil() as u64
    } else {
      1
    };
    let current_page = if total_results > 0 { 0 } else { 1 };
    (total_pages, current_page)
  } else {
    let pages_str = crate::logic::plain_parse(&html, &*PAGINATION_PAGES)?;
    let mut pages_split = pages_str.split(' ');

    let current_page: u64 = pages_split
      .nth(1)
      .ok_or_else(|| Error::invalid_content("4 items in pages string", None))?
      .parse()
      .map_err(Error::InvalidNumber)?;

    let total_pages: u64 = pages_split
      .nth(1)
      .ok_or_else(|| Error::invalid_content("4 items in pages string", None))?
      .parse()
      .map_err(Error::InvalidNumber)?;
    (total_pages, current_page)
  };

  Ok(Pagination {
    current_page,
    total_pages,
    total_results,
  })
}

crate fn parse_no_results<'a>(html: &Html) -> bool {
  html.select(&*NO_RESULTS)
    .next()
    .map(|x| x.text().collect::<String>() == "Your search yielded no results.")
    .unwrap_or(false)
}
