//! Textual version of the game 2048.

extern crate ansi_term as ansi ;
extern crate rand ;
#[macro_use] extern crate conrod;

mod common ;
pub mod cursor ;
pub mod clap ;
mod grid ;
pub mod frame ;

pub use common::{ Dir, Evolution, Seed } ;
pub use grid::{ Cell, Grid } ;

/// Entry point.
fn main() {
  use std::process::exit ;

  // Getting seed and painter from command line arguments.
  let (seed, painter) = match clap::parse() {
    Ok( (seed, painter) ) => (seed, painter),
    Err( (e, painter) ) => {
      println!("{}\n> {}", painter.error("Error:"), e) ;
      exit(2)
    },
  } ;

  frame::rendering_loop_user(seed, painter)
}

