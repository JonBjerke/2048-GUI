/// # Program for a 2048 GUI

/// ## Crates and Aspects of Crates Used
//Skeleton crate for a 2048 GUI.
extern crate lib_2048 ;
//Added Code
extern crate piston_window;
//Needed from skeleton crate
pub use lib_2048::{ Seed, Grid, Dir, Evolution, Cell } ;
//Added code
pub use piston_window::*;
pub type Color = [f32; 4] ;

//Alternate Color Theme

// static BACKGROUND: Color = [1.0, 1.0, 1.0, 1.0] ;
// static SCOREBOARD:  Color = [0.0, 0.0, 0.0, 1.0] ;
// static RESET: Color = [0.0, 0.0, 0.0, 1.0];
// static DEFAULTCELLBLACK:  Color = [0.0, 0.0, 0.0, 1.0] ;
// static CELLYELLOW:  Color = [1.0, 1.0, 0.0, 1.0] ;
// static CELLRED:  Color = [1.0, 0.0, 0.0, 1.0] ;
// static CELLGREEN:  Color = [0.0, 1.0, 0.0, 1.0] ;
// static CELLLIGHTBLUE:  Color = [0.0, 1.0, 1.0, 1.0] ;
// static CELLPINK:  Color = [1.0, 0.0, 1.0, 1.0] ;
// static CELLBLUE:  Color = [0.0, 0.5, 1.0, 1.0] ;

// fn color_of_pow(pow: u32) -> Color {
//   match pow {
//     0 => DEFAULTCELLBLACK,
//     1 => CELLYELLOW,
//     2 => CELLRED,
//     3 => CELLGREEN,
//     4 => CELLLIGHTBLUE,
//     5 => CELLPINK,
//     6 => CELLBLUE,
//     7 => CELLYELLOW,
//     8 => CELLRED,
//     9 => CELLGREEN,
//     10 => CELLLIGHTBLUE,
//     11 => CELLPINK,
//     12 => CELLBLUE,
//     _ => CELLYELLOW,
//   }
// } 


/// ## Colors Used for the GUI

/// Colors for the Window and Text
static BACKGROUND: Color = [0.3, 0.3, 0.3, 1.0] ;
static SCOREBOARD:  Color = [0.0, 0.0, 0.0, 1.0] ;
static RESET: Color = [0.0, 0.0, 0.0, 1.0];
static DEFAULTCELLBLACK:  Color = [0.0, 0.0, 0.0, 1.0] ;
static WHITE: Color = [1.0, 1.0, 1.0, 1.0] ;
/// Colors for the Cells of the Grid
static CELLYELLOW1:  Color = [1.0, 1.0, 0.0, 1.0] ;
static CELLYELLOW2:  Color = [1.0, 0.8, 0.0, 1.0] ;
static CELLYELLOW3:  Color = [1.0, 0.6, 0.0, 1.0] ;
static CELLORANGE1:  Color = [1.0, 0.4, 0.0, 1.0] ;
static CELLORANGE2:  Color = [1.0, 0.2, 0.0, 1.0] ;
static CELLRED1:  Color = [1.0, 0.0, 0.0, 1.0] ;
static CELLRED2: Color = [0.8, 0.0, 0.0, 1.0] ;
static CELLRED3: Color = [0.6, 0.0, 0.0, 1.0] ;
static CELLRED4:  Color = [0.4, 0.0, 0.0, 1.0] ;
static CELLLIGHTBLUE:  Color = [0.0, 1.0, 1.0, 1.0] ;
static CELLPINK:  Color = [1.0, 0.0, 1.0, 1.0] ;
static CELLBLUE:  Color = [0.0, 0.5, 1.0, 1.0] ;


/// ## Functions for the GUI

/// Matches the generated number for the cell to the appropriate color
fn color_of_pow(pow: u32) -> Color {
  match pow {
    0 => DEFAULTCELLBLACK,
    1 => CELLYELLOW1,
    2 => CELLYELLOW2,
    3 => CELLYELLOW3,
    4 => CELLORANGE1,
    5 => CELLORANGE2,
    6 => CELLRED1,
    7 => CELLRED2,
    8 => CELLRED3,
    9 => CELLRED4,
    10 => CELLLIGHTBLUE,
    11 => CELLBLUE,
    12 => CELLPINK,
    _ => CELLYELLOW1,
  }
}

/// Matches the cells with the correct position on the grid
fn pow_of(grid: & Grid, row: usize, col: usize) -> u32 {
  match grid.grid()[row][col] {
    None => 0,
    Some(ref cell) => cell.pow(),
  }
}

/// Generates the new number of the cell
fn value( pow: u32 ) -> u32{
  let mut value = 1;
  for x in 0..pow {
    value = 2 * value;
  }
  return value;
}

/// Displays the grid inside the piston window
fn display_grid<E: GenericEvent>(e: & E, grid: & Grid, window: & mut PistonWindow, glyphs: & mut Glyphs) {
  window.draw_2d(e, |c, g| {
            // Draws the basic rectangles and colors the background
            clear(BACKGROUND, g);
            rectangle(SCOREBOARD, 
                      [20.0, 20.0, 140.0, 60.0], 
                      c.transform, g);
            rectangle(RESET,
                      [180.0, 20.0, 140.0, 60.0],
                      c.transform, g);
            // Iterates through the row and columns to generate the color & number
            // for each cell
            for row in 0..4 {
              for col in 0..4 {
                rectangle(
                  color_of_pow( pow_of(grid, row, col) ), //Grey
                  [
                    20.0 + (col as f64) * (60.0 + 20.0),
                    100.0 + (row as f64) * 80.0,
                    60.0, 60.0
                  ],
                  c.transform, g
                ) ;
                let cell_pow = format!("{}", value(pow_of(grid, row, col)));
                let transform = c.transform.trans(
                  45.0 + (col as f64) * (60.0 + 20.0) - (pow_of(grid, row, col) as f64),
                  140.0 + (row as f64) * 80.0) ;
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 20).draw(
                  &cell_pow, glyphs, & c.draw_state, transform, g
                ) ;
              }
            } ;

            let score = format!("{}", grid.score()); 
            // Displays the score for the game
            let transform = c.transform.trans(20.0, 76.0) ;
            text::Text::new_color(WHITE, 20).draw(
              &score, glyphs, & c.draw_state, transform, g
            ) ;
            // Displays the score label for the game
            let transform = c.transform.trans(20.0, 36.0) ;
            text::Text::new_color(WHITE, 20).draw(
              "Score", glyphs, & c.draw_state, transform, g
            ) ; 
            // Displays instructions on how to reset the game
            let transform = c.transform.trans(188.0, 36.0) ;
            text::Text::new_color(WHITE, 15).draw(
              "Press 'r' to Reset", glyphs, & c.draw_state, transform, g
            ) ;
            // Displays instructions on how to quit the game
            let transform = c.transform.trans(188.0, 66.0) ;
            text::Text::new_color(WHITE, 15).draw(
              "Press 'Esc' to Quit", glyphs, & c.draw_state, transform, g
            ) ;
      });
}

/// Reads the user's input and matches it with the appropriate direction to execute
fn read_user_input(button: Button) -> Option<Dir> {  
  use keyboard::Key ;
  match button {
    Button::Keyboard(key) => match key {
      Key::Up | Key::W => Some( Dir::Up ),
      Key::Down | Key::S => Some( Dir::Dw ),
      Key::Left | Key::A => Some( Dir::Lf ),
      Key::Right | Key::D => Some( Dir::Rg ),
      _ => None,
    },
    _ => None
  }
}

/// Checks if the user's input was to reset the game
fn check_reset(button: Button, mut window: & mut PistonWindow) {
  use keyboard::Key ;
  match button {
    Button::Keyboard(key) => match key {
      Key::R => reset(& mut window),
      _ => (),
    },
    _ => () 
  }
}

/// Resets the game
fn reset (mut window: & mut PistonWindow) {
  game(& mut window);
}

/// Checks if the user's input was to exit the game
fn check_end(button: Button){
  use keyboard::Key ;
  match button {
      Button::Keyboard(key) => match key {
      Key::Escape => end(),
      _ => (),
    },
    _ => () 
  }
}

/// Exits the game
fn end(){
  use std::process::exit ;
  exit(0);
}

/// Takes the window as a parameter, starts and runs the game
fn game(mut window: & mut PistonWindow) {
  let font_path = "fonts/NotoSans-Bold.ttf" ;
  let mut glyphs = Glyphs::new(font_path, window.factory.clone()).unwrap() ;
  let seed = Seed::mk() ;
  // Create initial grid.
  let mut grid = Grid::mk(seed) ;
  grid.spawn() ;
  // Continues to generate as long as there is a game to play
  while let Some(event) = window.next() {
    match event {
      Event::Update(_) => (),
      e @ Event::Render(_) => display_grid(& e, & grid, & mut window, & mut glyphs),
      // Checks input from the user
      Event::Input( Input::Press(button) ) => {
        // Checks for reset or quit the game
        check_reset(button, & mut window);
        check_end(button);
        // Checks for direction move
        match read_user_input(button) {
          None => (),
          Some(dir) => {
            let evolution = match dir {
              Dir::Up => grid.up(),
              Dir::Dw => grid.down(),
              Dir::Lf => grid.left(),
              Dir::Rg => grid.right(),
            } ;
            // Checks if grid can move and shifts and merges cells if it can move
            match evolution {
              Evolution::Nothing => (),
              Evolution::Moved => {
                let could_spawn = grid.spawn();
                assert!( could_spawn );
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

/// Main function for GUI to start running
fn main() {
  //Creates a Window
  let mut window: PistonWindow = WindowSettings::new("2048", (340, 420)) //x,y
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
  // Starts the game
  game(& mut window);  
}
