use failure::Fail;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "couldn't find expected element on the lodestone: {}", _0)]
  MissingElement(String),
  #[fail(display = "the content scraped from the lodestone was invalid: {}", _0)]
  InvalidContent(String),
  #[fail(display = "invalid page (1 through {} available)", _0)]
  InvalidPage(u64),

  #[fail(display = "invalid number: {}", _0)]
  InvalidNumber(std::num::ParseIntError),
  #[fail(display = "invalid url: {}", _0)]
  InvalidUrl(url::ParseError),
}

impl Error {
  pub fn missing_element(select: &scraper::Selector) -> Self {
    use cssparser::ToCss;
    let css = select.selectors.iter().map(ToCss::to_css_string).collect::<Vec<_>>().join(" ");
    Error::MissingElement(css)
  }

  pub fn invalid_content(expecting: &str, found: Option<&str>) -> Self {
    let s = match found {
      Some(f) => format!("expecting `{}`, found `{}`", expecting, f),
      None => format!("expecting `{}`", expecting),
    };
    Error::InvalidContent(s)
  }
}
