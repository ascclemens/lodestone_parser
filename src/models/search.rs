pub mod character;
pub mod free_company;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pagination {
  pub current_page: u64,
  pub total_pages: u64,
  pub total_results: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paginated<T> {
  pub pagination: Pagination,
  pub results: Vec<T>,
}
