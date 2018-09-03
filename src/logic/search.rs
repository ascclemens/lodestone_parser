use crate::{
  error::*,
  models::search::Pagination,
};

use scraper::Html;

pub mod character;
pub mod free_company;

selectors!(
  PAGINATION_TOTAL => ".parts__total";
  PAGINATION_PAGES => ".btn__pager__current";
);

crate fn parse_pagination(html: &Html) -> Result<Pagination> {
  let total_str = crate::logic::plain_parse(&html, &*PAGINATION_TOTAL)?;
  let total_results: u64 = total_str
    .split(' ')
    .next()
    .unwrap() // will always have a first element
    .parse()
    .map_err(Error::InvalidNumber)?;

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

  Ok(Pagination {
    current_page,
    total_pages,
    total_results,
  })
}
