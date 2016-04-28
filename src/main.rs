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

static BACKGROUND: Color = [255.0, 255.0, 255.0, 1.0] ;
static SCOREBOARD:  Color = [0.0, 0.0, 0.0, 1.0] ;
static RESET: Color = [0.0, 0.0, 0.0, 1.0];
static DEFAULTCELL:  Color = [0.0, 0.0, 0.0, 1.0] ;
static CELLYELLOW:  Color = [255.0, 255.0, 0.0, 1.0] ;
static CELLRED:  Color = [255.0, 0.0, 0.0, 1.0] ;
static CELLGREEN:  Color = [0.0, 255.0, 0.0, 1.0] ;
static CELLLIGHTBLUE:  Color = [0.0, 255.0, 255.0, 1.0] ;
static CELLPINK:  Color = [255.0, 0.0, 255.0, 1.0] ;
static CELLBLUE:  Color = [0.0, 0.0, 255.0, 1.0] ;



fn color_of_pow(pow: u32) -> Color {
  match pow {
    0 => DEFAULTCELL,
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

// fn main() {
//     // Use the open function to load an image from a Path.
//     // ```open``` returns a dynamic image.
//     let img = image::open(&Path::new("test.jpg")).unwrap();

//     // The dimensions method returns the images width and height
//     println!("dimensions {:?}", img.dimensions());

//     // The color method returns the image's ColorType
//     println!("{:?}", img.color());

//     let ref mut fout = File::create(&Path::new("test.png")).unwrap();

//     // Write the contents of this image to the Writer in PNG format.
//     let _ = img.save(fout, image::PNG).unwrap();
// }

fn pow_of(grid: & Grid, row: usize, col: usize) -> u32 {
  match grid.grid()[row][col] {
    None => 0,
    Some(ref cell) => cell.pow(),
  }
}

/// Displays the grid using for instance piston.
fn display_grid(grid: & Grid, window: & mut PistonWindow) {
  
  
  if let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear(BACKGROUND, g); 
            
            rectangle(SCOREBOARD, 
                      [20.0, 20.0, 140.0, 60.0], 
                      c.transform, g);
            rectangle(RESET,
                      [180.0, 20.0, 140.0, 60.0],
                      c.transform, g);
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
              }
            }

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

  img::open("blah") ;

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

  // Create initial grid.
  let mut grid = Grid::mk(seed) ;
  grid.spawn() ;

  // let mut count = 0 ;

  // Rendering loop.
  'rendering: loop {
    use Dir::* ;
    use Evolution::* ;

    // count = (count + 1) % 1000 ;

    // println!("count: {}", count) ;
  
    display_grid(& grid, & mut window) ;


    
    grid.right(); 
    //grid.spawn(); 
    grid.left(); 
    
    grid.right(); 
    
    grid.left(); 
   
    grid.right(); 
   
    grid.up();
    
    // match count {
    //   250 => { grid.up() ; () },
    //   500 => { grid.spawn() ; () },
    //   750 => { grid.left() ; () },
    //   999 => { grid.spawn() ; () },
    //   _ => (),
    // }

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

