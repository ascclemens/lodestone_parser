pub mod character;

#[derive(Debug, Serialize)]
pub struct Pagination {
  pub current_page: u64,
  pub total_pages: u64,
  pub total_results: u64,
}

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
  pub pagination: Pagination,
  pub results: Vec<T>,
}
