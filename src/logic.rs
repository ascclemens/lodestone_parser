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
