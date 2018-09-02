#![feature(macro_at_most_once_rep)]

#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

pub mod error;
pub mod logic;
pub mod models;
