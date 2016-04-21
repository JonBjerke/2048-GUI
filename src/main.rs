//! Skeleton crate for a 2048 GUI.

extern crate lib_2048 ;
//Added Code
extern crate ansi_term as ansi ;
extern crate rand ;
#[macro_use] extern crate conrod;
extern crate piston_window;
// 


pub use lib_2048::{ Seed, Grid, Dir, Evolution, Cell } ;

//Added code
use conrod::{Canvas, Theme, Widget };//,color};
use piston_window::*;//{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, clear};
//

/// Displays the grid using for instance piston.
fn display_grid(_grid: & Grid, window: & mut PistonWindow) {
  
  while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color = dark Grey
                      [0.0, 0.0, 640.0, 580.0], // rectangle
                      c.transform, g);
            rectangle([0.6, 0.6, 0.6, 0.6], //  light grey
                      [20.0, 100.0, 380.0, 380.0], // rectangle
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // black
                      [100.0, 100.0, 20.0, 380.0], // c1
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color
                      [200.0, 100.0, 20.0, 380.0], // c2
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color
                      [300.0, 100.0, 20.0, 380.0], // c3
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color
                      [20.0, 180.0, 380.0, 20.0], // r1
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color
                      [20.0, 280.0, 380.0, 20.0], // r2
                      c.transform, g);
            rectangle([0.4, 0.4, 0.4, 1.0], // color
                      [20.0, 380.0, 380.0, 20.0], // r3
                      c.transform, g);
            rectangle([0.6, 0.6, 0.6, 1.0], //  light grey
                      [220.0, 20.0, 180.0, 60.0], // current score
                      c.transform, g);
            rectangle([0.6, 0.6, 0.6, 1.0], //  light grey
                      [20.0, 20.0, 180.0, 60.0], // high score
                      c.transform, g);

      });
  }

  //panic!("display is not implemented")
  //Display new score
  //grid.score();
  //grid.grid(); // vector of vector or cells
  //cell
  //println!("got to display_grid");
  //extra credit for defining a trait for grid that will implement the key press
  //
}

/// Read user input (up left down right). User may also want to exit, reset,
/// *etc.*
fn read_user_input() -> Dir {
  //Evolution goes here
  panic!("user interaction is not implemented")
}

fn main() {
  // Just like in minecraft, randomness is dictated by a seed.
  // This creates a random seed.
  let seed = Seed::mk() ;

  // Eventually you may want to allow the user to provide their own seed,
  // if they want to play the same game again.
  // That is, the `2` and `4` tiles will spawn at exactly the same place **if
  // the player makes the same moves**.
  // let seed = Seed::of_str( <user_provided_string> ) ;

  //create a Window
  let mut window: PistonWindow = WindowSettings::new("2048", (420, 500)) //x,y
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
  use std::process::exit ;

  // Create initial grid.
  let mut grid = Grid::mk(seed) ;
  grid.spawn() ;

  // Rendering loop.
  'rendering: loop {
    use Dir::* ;
    use Evolution::* ;
  
    display_grid(& grid, & mut window) ;
    // 1) display new grid 
    // 2) ask for user input
    // 3) the game calculates new score and tile locations
    // 4) loop ends
    // (could put 1 at the end once all other parts are implemented.)

    // // What is the user asking you to do?
    // let evolution = match read_user_input() {
    // let evolution = match AI{
    //   Up => grid.up(),
    //   Dw => grid.down(),
    //   Rg => grid.right(),
    //   Lf => grid.left(),
    // } ;

    // // The evolution tells you what happened.
    // match evolution {
    //   // Nothing happened (no move no merge).
    //   Nothing => (),
    //   // Some tiles moved but no merge.
    //   Moved => grid.spawn(),
    //   // Merged some tiles, yields the score of the tiles merged.
    //   Merged(_score) => grid.spawn(),//possible animation
    // }
  }
}
