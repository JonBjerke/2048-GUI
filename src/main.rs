//! Textual version of the game 2048.

extern crate ansi_term as ansi ;
extern crate rand ;
#[macro_use] extern crate conrod;
extern crate piston_window;


mod common ;
pub mod cursor ;
pub mod clap ;
mod grid ;
pub mod frame ;
//
use conrod::{Canvas, Theme, Widget, color};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};
//

pub use common::{ Dir, Evolution, Seed } ;
pub use grid::{ Cell, Grid } ;

//
type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;
//

/// Entry point.
fn main() {
  /*/HERE 
  let window: PistonWindow =
        WindowSettings::new("Canvas Demo", [800, 600])
            .exit_on_esc(true).vsync(true).build().unwrap();
  //TO HERE*/
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

