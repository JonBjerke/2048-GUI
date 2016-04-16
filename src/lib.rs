//! 2048 structures and functions.

extern crate ansi_term as ansi ;
extern crate rand ;
#[macro_use] extern crate conrod;

mod common ;
mod grid ;
mod frame ;
mod cursor ;

pub mod clap ;

/// Rendering types and loop.
pub mod rendering {
  pub use super::frame::* ;
}

pub use common::{ Dir, Evolution, Seed } ;
pub use grid::{ Cell, Grid } ;