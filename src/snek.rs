use std::collections::HashMap;
use std::fmt;

use colored::{ColoredString, Colorize};

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::{Canvas};
use sdl2::surface::Surface;
use sdl2::video::{Window};

#[derive(Debug)]
enum SnekDirection {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Debug, PartialEq)]
enum SnekObject {
    Berry,
    Empty,
    Head,
    Segment,
    Rock,
    Wall,
}

#[derive(Debug)]
pub struct SnekGame {
    pub game_over: bool,
    xsize: usize,
    ysize: usize,
    // FIXME - is board needed?  Maybe just have Hashmaps of objects.
    board: HashMap<(usize, usize), SnekObject>,

    snek_head_pos: (i32, i32),
    snek_head_dir: SnekDirection,
    snek_length: usize,
    snek_seg_pos: Vec<(usize, usize)>
}

impl SnekGame {

    pub fn new(xsize: usize, ysize: usize) -> SnekGame {
        let mut board: HashMap<(usize, usize), SnekObject> = HashMap::new();

        let snek_head_pos = (10, 10);
        let snek_head_dir = SnekDirection::East;
        let snek_length = 4;
        let mut game = SnekGame {
                game_over: false,
                xsize,
                ysize,
                board,
                snek_head_pos,
                snek_head_dir,
                snek_length,
                snek_seg_pos: Vec::new(),
        };
        game.set_cell(snek_head_pos.0 as usize, snek_head_pos.1 as usize, SnekObject::Head);
        game.set_cell(14, 10, SnekObject::Berry);
        game.set_cell(20, 10, SnekObject::Rock);
        game
    }

    // TODO: Is this actually needed?
    fn coord_to_index(&self, x: usize, y: usize) -> usize {
        y * self.xsize + x
    }

    /* Set a cell on the gameboard to a particular object.  If
     * the cell is outside the bounds of the board, this is a no-op,
     * although should possibly be an error?
     */
    fn set_cell(&mut self, x: usize, y: usize, obj: SnekObject) {
        /* Sanity check on bounds. */
        if x < 0 || x >= self.xsize || y < 0 || y > self.ysize {
            return;
        }
      
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        /* If the cell was already occupied, clear it. */
        if self.get_cell(x as i32, y as i32) != SnekObject::Empty {
            self.board.remove(&(x, y));
        }


        /* Populate the cell with the specified SnekObject. */
        self.board.insert((x, y), obj.clone());
    }

    /* Get the contents of a cell on the gameboard.  If the
     * cell is outside the gameboard, it is treated as a wall.
     * The x and y coordinates are passed as signed instead of
     * usize to make it easy to iterate over adjacent cells
     * which might land outside of the board.
     */
    fn get_cell(&self, x: i32, y: i32) -> SnekObject {
        let ysize: i32 = self.ysize.try_into().unwrap();
        let xsize: i32 = self.xsize.try_into().unwrap();
        if x < 0 || x >= xsize || y < 0 || y > ysize {
            return SnekObject::Wall;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        if let Some(cell) = self.board.get(&(x, y)) {
            cell.clone()
        } else {
            SnekObject::Empty
        }
    }

    // Called every game step
    pub fn step(&mut self) {
        // Check if game is in end state.  No-op if true.
        if self.game_over {
            return;
        }

        // Snek moves one step in facing direction
        let (x, y) = self.snek_head_pos;
        let mut xnew = x;
        let mut ynew = y;
        match self.snek_head_dir {
            SnekDirection::North => {
                ynew -= 1;
            }
            SnekDirection::East => {
                xnew += 1;
            }
            SnekDirection::West => {
                xnew -= 1;
            }
            SnekDirection::South => {
                ynew += 1;
            }
        }

        match self.get_cell(xnew, ynew) {
            SnekObject::Berry => {
                println!("Snake ate a berry @ ({}, {})!", xnew, ynew);
                self.snek_length += 1;
                // When a berry is eaten, snake head moves one and leaves a new
                // segment behind.
            },
            SnekObject::Wall => {
                println!("Snake hit the wall @ ({}, {})!", xnew, ynew);
                self.game_over = true;
                return;
            },
            SnekObject::Rock => {
                println!("Snake hit a rock @ ({}, {})!", xnew, ynew);
                self.game_over = true;
                return;
            },
            _ => { /* WARK! */ },
        }
        self.board.remove(&(x.try_into().unwrap(), y.try_into().unwrap()));
        self.set_cell(xnew.try_into().unwrap(), ynew.try_into().unwrap(), SnekObject::Head);
        self.snek_head_pos = (xnew, ynew);

    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.draw_grid(canvas);
        
    }

    fn draw_head(&self, canvas: &mut Canvas<Window>) {

    }

    fn draw_segments(&self, canvas: &mut Canvas<Window>) {
        // iterate over segment vector, drawing each in
        // alternating colors.  
    }

    fn draw_berries(&self, canvas: &mut Canvas<Window>) {

    }

    fn draw_rocks(&self, canvas: &mut Canvas<Window>) {

    }

    /* Draw the game grid. */
    pub fn draw_grid(&self, canvas: &mut Canvas<Window>) {
        const SPACING: usize = 32;
        const GRID_COLOR: Color = Color::RGB(0, 255, 0);
        const WALL_COLOR: Color = Color::RGB(255, 0, 0);

        let orig_color = canvas.draw_color();

        // Get surface dimensions, coerce into i32 so we can use them as Points.
        let (xsize, ysize) = canvas.output_size().unwrap();
        let xsize: i32 = xsize.try_into().unwrap();
        let ysize: i32 = ysize.try_into().unwrap();

        // Draw the grid
        canvas.set_draw_color(GRID_COLOR);
        for x in (0..xsize).step_by(SPACING) {
            canvas.draw_line(Point::new(x, 0), Point::new(x, ysize)).unwrap();
        }
        for y in (0..ysize).step_by(SPACING) {
            canvas.draw_line(Point::new(0, y), Point::new(xsize, y)).unwrap();
        }

        // Draw the walls
        canvas.set_draw_color(WALL_COLOR);
        canvas.draw_line(Point::new(0, 0), Point::new(xsize - 1, 0)).unwrap();
        canvas.draw_line(Point::new(0, 0), Point::new(0, ysize - 1)).unwrap();
        canvas.draw_line(Point::new(xsize - 1, 0), Point::new(xsize - 1, ysize - 1)).unwrap();
        canvas.draw_line(Point::new(0, ysize - 1), Point::new(xsize - 1, ysize - 1)).unwrap();
        
        // Restore original color
        canvas.set_draw_color(orig_color);
    }
}


impl fmt::Display for SnekGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                let obj = self.get_cell(x as i32, y as i32);
                match obj {
                    SnekObject::Berry => {
                        write!(f, "{}", "▄".to_string().red().bold())?
                    },
                    SnekObject::Head => {
                        let v = match self.snek_head_dir {
                            SnekDirection::North => "↑",
                            SnekDirection::East => "→",
                            SnekDirection::West => "←",
                            SnekDirection::South => "↓",
                        };
                        let v = v.to_string().green().bold();
                        write!(f, "{}", v)?
                    },
                    SnekObject::Rock => {
                        write!(f, "{}", "█".to_string().white().bold())?
                    },
                    SnekObject::Segment => {
                        write!(f, "{}", "■".to_string().yellow().bold())?
                    },
                                        
                    _ => write!(f, "_")?,
                }
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

