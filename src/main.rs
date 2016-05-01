//! Skeleton crate for a 2048 GUI.

extern crate lib_2048 ;
//Added Code
extern crate ansi_term as ansi ;
extern crate rand ;
extern crate conrod;
extern crate piston_window;
extern crate image as img ;
extern crate opengl_graphics;
// 

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

pub use lib_2048::{ Seed, Grid, Dir, Evolution, Cell } ;

//Added code
use conrod::{Canvas, Theme, Widget };//,color};
pub use piston_window::*;//{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, clear};
//

pub type Color = [f32; 4] ;

static BACKGROUND: Color = [1.0, 1.0, 1.0, 1.0] ;
static SCOREBOARD:  Color = [0.0, 0.0, 0.0, 1.0] ;
static RESET: Color = [0.0, 0.0, 0.0, 1.0];
static DEFAULTCELLBLACK:  Color = [0.0, 0.0, 0.0, 1.0] ;
static CELLYELLOW:  Color = [1.0, 1.0, 0.0, 1.0] ;
static CELLRED:  Color = [1.0, 0.0, 0.0, 1.0] ;
static CELLGREEN:  Color = [0.0, 1.0, 0.0, 1.0] ;
static CELLLIGHTBLUE:  Color = [0.0, 1.0, 1.0, 1.0] ;
static CELLPINK:  Color = [1.0, 0.0, 1.0, 1.0] ;
static CELLBLUE:  Color = [0.0, 0.5, 1.0, 1.0] ;



fn color_of_pow(pow: u32) -> Color {
  match pow {
    0 => DEFAULTCELLBLACK,
    1 => CELLYELLOW,
    2 => CELLRED,
    3 => CELLGREEN,
    4 => CELLLIGHTBLUE,
    5 => CELLPINK,
    6 => CELLBLUE,
    7 => CELLYELLOW,
    8 => CELLRED,
    9 => CELLGREEN,
    10 => CELLLIGHTBLUE,
    11 => CELLPINK,
    12 => CELLBLUE,
    _ => CELLYELLOW,
  }
}
//ANOTHER THEME
//static BACKGROUND: Color = [0.3, 0.3, 0.3, 1.0] ;
// static SCOREBOARD:  Color = [0.0, 0.0, 0.0, 1.0] ;
// static RESET: Color = [0.0, 0.0, 0.0, 1.0];
// static DEFAULTCELLBLACK:  Color = [0.0, 0.0, 0.0, 1.0] ;

// static WHITE: Color = [1.0, 1.0, 1.0, 1.0] ;

// static CELLYELLOW:  Color = [1.0, 1.0, 0.0, 1.0] ;
// static CELLRED:  Color = [1.0, 0.8, 0.0, 1.0] ;
// static CELLGREEN:  Color = [1.0, 0.6, 0.0, 1.0] ;
// static CELLLIGHTBLUE:  Color = [1.0, 0.4, 0.0, 1.0] ;
// static CELLPINK:  Color = [1.0, 0.2, 0.0, 1.0] ;
// static CELLBLUE:  Color = [1.0, 0.0, 0.0, 1.0] ;
// static CELLYELLOW2: Color = [0.8, 0.0, 0.0, 1.0] ;
// static CELLRED2: Color = [0.6, 0.0, 0.0, 1.0] ;
// static CELLGREEN2:  Color = [0.4, 0.0, 0.0, 1.0] ;

// fn color_of_pow(pow: u32) -> Color {
//   match pow {
//     0 => DEFAULTCELLBLACK,
//     1 => CELLYELLOW,
//     2 => CELLRED,
//     3 => CELLGREEN,
//     4 => CELLLIGHTBLUE,
//     5 => CELLPINK,
//     6 => CELLBLUE,
//     7 => CELLYELLOW2,
//     8 => CELLRED2,
//     9 => CELLGREEN2,
//     10 => CELLLIGHTBLUE,
//     11 => CELLPINK,
//     12 => CELLBLUE,
//     _ => CELLYELLOW,
//   }
// }

//path can just be a string
//image: GlTexture::from_path(Path::new("bin/assets/digits.png")).unwrap(),

// Image::new_color([color[0], color[1], color[2], 1.0])
//                 .src_rect([(*digit * DIGITS_WIDTH as u32) as i32, 0, DIGITS_WIDTH as i32, DIGITS_HEIGHT as i32])
//                 .rect([x, y, width, height])
//                 .draw(&self.image,
//                       default_draw_state(),
//                       c.transform,
//                       gl);

////https://github.com/PistonDevelopers/image

// pub use std::path::Path;

// fn path_of_image(pow: u32) -> Path {

//   let pathEmpty = Path::new("pics/empty.png");
//   let path2 = Path::new("pics/2.png");
//   let path128 = Path::new("pics/128.png");
//   let path8192 = Path::new("pics/8192.png");

//   match pow {
//     0 => pathEmpty,
//     1 => path2,
//     2 => path128,
//     _ => path8192,
//   }
// }

// extern crate image;

// use std::fs::File;
// use std::path::Path;

// use image::GenericImage;

fn pow_of(grid: & Grid, row: usize, col: usize) -> u32 {
  match grid.grid()[row][col] {
    None => 0,
    Some(ref cell) => cell.pow(),
  }
}

fn value( pow: u32 ) -> u32{
  let i = 0;
  let mut value = 1;
  for i in 0..pow {
    value = 2 * value;
  }
  return value;
}

/// Displays the grid using for instance piston.
fn display_grid<E: GenericEvent>(e: & E, grid: & Grid, window: & mut PistonWindow, glyphs: & mut Glyphs) {
  
  
  window.draw_2d(e, |c, g| {
            clear(BACKGROUND, g); 
            
            rectangle(SCOREBOARD, 
                      [20.0, 20.0, 140.0, 60.0], 
                      c.transform, g);
            rectangle(RESET,
                      [180.0, 20.0, 140.0, 60.0],
                      c.transform, g);

            let mut cell_pow = format!("{}", grid.score());

            for row in 0..4 {
              for col in 0..4 {
                rectangle(
                  color_of_pow( pow_of(grid, row, col) ), //  grey
                  [
                    20.0 + (col as f64) * (60.0 + 20.0),
                    100.0 + (row as f64) * 80.0,
                    60.0, 60.0
                  ],
                  c.transform, g
                ) ;
                
                cell_pow = format!("{}", value(pow_of(grid, row, col)));
                let transform = c.transform.trans(
                  45.0 + (col as f64) * (60.0 + 20.0) - (pow_of(grid, row, col) as f64),
                  140.0 + (row as f64) * 80.0) ;
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 20).draw(
                  &cell_pow, glyphs, & c.draw_state, transform, g
                ) ;
                
              }
            } ;

            let mut score = format!("{}", grid.score()); 

            let transform = c.transform.trans(20.0, 76.0) ;
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 20).draw(
              &score, glyphs, & c.draw_state, transform, g
            ) ;

            let transform = c.transform.trans(20.0, 36.0) ;
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 20).draw(
              "Score", glyphs, & c.draw_state, transform, g
            ) ;

            let transform = c.transform.trans(190.0, 55.0) ;
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 15).draw(
              "Push 'r' to Reset", glyphs, & c.draw_state, transform, g
            ) ;
      });
}

/// Read user input (up left down right). User may also want to exit, reset,
/// *etc.*
fn read_user_input(button: Button) -> Option<Dir> {  
  use keyboard::Key ;
  match button {
    Button::Keyboard(key) => match key {
      Key::Up | Key::W => Some( Dir::Up ),
      Key::Down | Key::S => Some( Dir::Dw ),
      Key::Left | Key::A => Some( Dir::Lf ),
      Key::Right | Key::D => Some( Dir::Rg ),
       //Key::R => reset();
      _ => None,
    },
    _ => None
  }
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
  let mut window: PistonWindow = WindowSettings::new("2048", (340, 420)) //x,y
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
  use std::process::exit ;

  let font_path = "fonts/NotoSans-Bold.ttf" ;
  let mut glyphs = Glyphs::new(font_path, window.factory.clone()).unwrap() ;

  // Create initial grid.
  let mut grid = Grid::mk(seed) ;

  grid.spawn() ;

  while let Some(event) = window.next() {
    match event {
      Event::Update(_) => (),
      e @ Event::Render(_) => display_grid(& e, & grid, & mut window, & mut glyphs),
      Event::Input( Input::Press(button) ) => {
        match read_user_input(button) {
          None => (),
          Some(dir) => {
            let evolution = match dir {
              Dir::Up => grid.up(),
              Dir::Dw => grid.down(),
              Dir::Lf => grid.left(),
              Dir::Rg => grid.right(),
            } ;
            match evolution {
              Evolution::Nothing => (),
              Evolution::Moved => {
                let could_spawn = grid.spawn() ;
                assert!( could_spawn )
              },
              Evolution::Merged(_) => {
                let could_spawn = grid.spawn() ;
                assert!( could_spawn )
              },
            }
          }
        }
      },
      _ => (),
    }
  }
}
