#[cfg(feature = "with_serde")] use serde_derive::{Deserialize, Serialize};

pub mod character;
pub mod free_company;
pub mod linkshell;

#[derive(Debug, Default)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct Pagination {
  pub current_page: u64,
  pub total_pages: u64,
  pub total_results: u64,
}

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(Deserialize, Serialize))]
pub struct Paginated<T> {
  pub pagination: Pagination,
  pub results: Vec<T>,
}
