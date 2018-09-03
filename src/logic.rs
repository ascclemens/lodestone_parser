use crate::error::*;

use scraper::Html;

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

crate fn plain_parse(html: &Html, select: &scraper::Selector) -> Result<String> {
  let string = html
    .select(select)
    .next()
    .ok_or(Error::missing_element(select))?
    .text()
    .collect();
  Ok(string)
}
