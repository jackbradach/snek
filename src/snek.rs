use std::collections::HashMap;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::{Canvas};
use sdl2::surface::Surface;
use sdl2::video::{Window};


enum SnekDirection {
    North,
    East,
    West,
    South,
}

#[derive(Clone)]
enum SnekObject {
    Head,
    Segment,
    Berry,
    Wall,
    Empty,
}

pub struct SnekGame {
    xsize: usize,
    ysize: usize,
    // FIXME - is board needed?  Maybe just have Hashmaps of objects.
    board: Vec<SnekObject>,

    snek_head_pos: (usize, usize),
    snek_head_dir: SnekDirection,
    snek_length: usize,
    snek_seg_pos: Vec<(usize, usize)>
}

impl SnekGame {

    pub fn new(xsize: usize, ysize: usize) -> SnekGame {
        let mut board: Vec<SnekObject> = vec![SnekObject::Empty; ysize * xsize];

        let snek_head_pos = (10, 10);
        let snek_head_dir = SnekDirection::East;
        let snek_length = 4;
        let mut game = SnekGame {
                xsize,
                ysize,
                board,
                snek_head_pos,
                snek_head_dir,
                snek_length,
                snek_seg_pos: Vec::new(),
        };
        game.set_cell(snek_head_pos.0, snek_head_pos.1, SnekObject::Head);
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
        if x < 0 || x >= self.xsize || y < 0 || y > self.ysize {
            return;
        }
        let index: usize = y * self.xsize + x;
        self.board[index] = obj;
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
        let index: usize = (y * xsize + x).try_into().unwrap();
        self.board[index].clone()
    }

    // Called every game step
    pub fn step(&mut self) {

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

struct Snek {
    direction: SnekDirection,
    segments: Vec<SnekSegments>,
}

impl Snek {

    fn new() -> Snek {
        // Snake tail should go opposite of direction.
        Snek {
            direction: SnekDirection::East,
            segments: Vec::new(),
        }
    }

    // This should take a rectangle surface the size of a game grid
    // and draw the snek shape or sprite to it.
    fn draw() {

    }


}

// Snake segments need to be able to follow previous
// segments.
struct SnekSegments {
    x: usize,
    y: usize,
}

impl SnekSegments {

}

