//! Skeleton crate for a 2048 GUI.

extern crate lib_2048 ;
extern crate piston ; 
extern crate graphics ; 
extern crate glutin_window ;
extern crate opengl_graphics ;

pub use lib_2048::{ Seed, Grid, Dir, Evolution } ;

/// Structure for game Window
pub struct Game {
  gl: GlGraphics
}

impl Game {
  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;

    let square = rectangle::square(0.0, 0.0, 50.0);

  }
}
/// Displays the grid using for instance piston.
fn display_grid(_grid: & Grid) {
  let mut window: Window = WindowSettings::new(
      "2048-Game",
      [200, 200]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut game = Game {
    gl: GlGraphics::new(opengl),
    rotation:0.0
  };

  let mut events = window.events();
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      game.render(&r);
    }
  }
  //panic!("display is not implemented")
}

/// Read user input (up left down right). User may also want to exit, reset,
/// *etc.*
fn read_user_input() -> Dir {
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

  // Create initial grid.
  let mut grid = Grid::mk(seed) ;

  // Rendering loop.
  'rendering: loop {
    use Dir::* ;
    use Evolution::* ;

    // Display the grid.
    display_grid(& grid) ;

    // What is the user asking you to do?
    let evolution = match read_user_input() {
      Up => grid.up(),
      Dw => grid.down(),
      Rg => grid.right(),
      Lf => grid.left(),
    } ;

    // The evolution tells you what happened.
    match evolution {
      // Nothing happened (no move no merge).
      Nothing => (),
      // Some tiles moved but no merge.
      Moved => (),
      // Merged some tiles, yields the score of the tiles merged.
      Merged(_score) => (),
    }
  }
}