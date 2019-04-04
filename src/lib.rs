#![feature(crate_visibility_modifier)]
#![allow(clippy::unreadable_literal)]

pub extern crate ffxiv_types;

#[cfg(feature = "logic")]
pub mod error;
#[cfg(feature = "logic")]
pub mod logic;

pub mod models;
crate mod util;

#[cfg(feature = "logic")]
pub use crate::logic::*;
