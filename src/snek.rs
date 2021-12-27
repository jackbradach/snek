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

enum SnekObject {
    SnekHead,
    SnekSegment,
    SnekBerry,
    SnekWall,
}

pub struct GameBoard {
    x_max: usize,
    y_max: usize,
    board: Vec<Option<SnekObject>>,
}

impl GameBoard {

    pub fn new() -> GameBoard {
        const X_MAX: usize = 50;
        const Y_MAX: usize = 50;
        let board: Vec<Option<SnekObject>> = Vec::with_capacity(Y_MAX * X_MAX);
        GameBoard {
            x_max: X_MAX,
            y_max: Y_MAX,
            board,
        }
    }

    // Called every game step
    // Propagates to the sub-objects so they can update.
    pub fn step(&mut self) {

    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let start = Point::new(0, 100);
        let end = Point::new(800, 100);

        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // let color = ;
        canvas.set_draw_color(pixels::Color::RGB(0, 255, 0));
        canvas.draw_line(start, end);
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
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

