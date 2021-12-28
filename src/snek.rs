use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels;
use sdl2::rect::Point;

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

pub struct GameBoard {
    x_max: usize,
    y_max: usize,
    board: Vec<SnekObject>,
}

impl GameBoard {

    pub fn new() -> GameBoard {
        const X_MAX: usize = 50;
        const Y_MAX: usize = 50;
        let board: Vec<SnekObject> = vec![SnekObject::Empty; Y_MAX * X_MAX];
        GameBoard {
            x_max: X_MAX,
            y_max: Y_MAX,
            board,
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> SnekObject {
        SnekObject::Empty
    }

    // Called every game step
    // Propagates to the sub-objects so they can update.
    pub fn step(&mut self) {

    }

    pub unsafe fn draw(&self, canvas: &mut Canvas<Window>) {
        // const SPACING: usize = 40;
        // static mut green: u8 = 0;
        // canvas.set_draw_color(pixels::Color::RGB(0, green, 0));
        // for x in (0..800).step_by(SPACING) {
        //     canvas.draw_line(Point::new(x, 0), Point::new(x, 599));
        // }
        // for y in (0..640).step_by(SPACING) {
        //     canvas.draw_line(Point::new(0, y), Point::new(799, y));
        // }
        // canvas.draw_line(Point::new(799, 0), Point::new(799, 599));
        // canvas.draw_line(Point::new(0, 599), Point::new(799, 599));
        
        // canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        // green = green.wrapping_add(1);
    }

    pub fn draw_grid(&self, canvas: &mut Canvas<Window>) {
        const SPACING: usize = 32;
        const GRID_COLOR: pixels::Color = pixels::Color::RGB(0, 255, 0);
        const WALL_COLOR: pixels::Color = pixels::Color::RGB(255, 0, 0);

        let orig_color = canvas.draw_color();

        // Get surface dimensions, coerce into i32 so we can use them as Points.
        let (xmax, ymax) = canvas.output_size().unwrap();
        let xmax: i32 = xmax.try_into().unwrap();
        let ymax: i32 = ymax.try_into().unwrap();

        // Draw the grid
        canvas.set_draw_color(GRID_COLOR);
        for x in (0..xmax).step_by(SPACING) {
            canvas.draw_line(Point::new(x, 0), Point::new(x, ymax)).unwrap();
        }
        for y in (0..ymax).step_by(SPACING) {
            canvas.draw_line(Point::new(0, y), Point::new(xmax, y)).unwrap();
        }

        // Draw the walls
        canvas.set_draw_color(WALL_COLOR);
        canvas.draw_line(Point::new(0, 0), Point::new(xmax - 1, 0)).unwrap();
        canvas.draw_line(Point::new(0, 0), Point::new(0, ymax - 1)).unwrap();
        canvas.draw_line(Point::new(xmax - 1, 0), Point::new(xmax - 1, ymax - 1)).unwrap();
        canvas.draw_line(Point::new(0, ymax - 1), Point::new(xmax - 1, ymax - 1)).unwrap();
        
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


}

// Snake segments need to be able to follow previous
// segments.
struct SnekSegments {
    x: usize,
    y: usize,
}

impl SnekSegments {

}

