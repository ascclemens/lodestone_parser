use crate::{
  error::*,
  models::{
    GrandCompany,
    character::GrandCompanyInfo,
  },
};

use scraper::{
  Html,
  ElementRef,
  node::Element,
};

macro_rules! selectors {
  ($($name:ident => $selector:expr);+$(;)?) => {
    lazy_static! {
      $(
        static ref $name: scraper::Selector = scraper::Selector::parse($selector).unwrap();
      )+
    }
  }
}

pub mod character;
pub mod free_company;
pub mod search;

pub use self::{
  character::parse as parse_character,
  free_company::parse as parse_free_company,
  search::*,
};

crate fn plain_parse(html: &Html, select: &scraper::Selector) -> Result<String> {
  let string = html
    .select(select)
    .next()
    .ok_or(Error::missing_element(select))?
    .text()
    .collect();
  Ok(string)
}

crate fn plain_parse_elem<'a>(html: ElementRef<'a>, select: &scraper::Selector) -> Result<String> {
  let string = html
    .select(select)
    .next()
    .ok_or(Error::missing_element(select))?
    .text()
    .collect();
  Ok(string)
}

crate fn parse_id<'a>(a: &'a Element) -> Result<u64> {
  let href = a.attr("href").ok_or_else(|| Error::invalid_content("href on link", None))?;
  let last = href
    .split('/')
    .filter(|x| !x.is_empty())
    .last()
    .ok_or_else(|| Error::invalid_content("href separated by `/`", Some(&href)))?;
  last.parse().map_err(Error::InvalidNumber)
}

crate fn parse_grand_company(text: &str) -> Result<GrandCompanyInfo> {
  let mut x = text.split(" / ");
  let gc_str = x
    .next()
    .ok_or_else(|| Error::invalid_content("gc/rank separated by `/`", Some(&text)))?;
  let grand_company = GrandCompany::parse(gc_str)
    .ok_or_else(|| Error::invalid_content("valid grand company", Some(&text)))?;
  let rank = x
    .next()
    .ok_or_else(|| Error::invalid_content("gc/rank separated by `/`", Some(&text)))?
    .to_string();
  Ok(GrandCompanyInfo {
    grand_company,
    rank,
  })
}
