#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use rand::Rng;
use colored::{Colorize};

use sdl2::pixels::{Color};
use sdl2::rect::Point;
use sdl2::render::{Canvas};
use sdl2::rect::Rect;
// use sdl2::surface::Surface;
use sdl2::video::{Window};

// TODO; have a struct for points in the game board.  Replace all x, y function calls.

#[derive(Clone, Debug, PartialEq)]
pub enum SnekDirection {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnekPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct SnekGame {
    pub game_over: bool,
    xsize: usize,
    ysize: usize,
    // FIXME - is board needed?  Maybe just have Hashmaps of objects.
    board: HashMap<SnekPosition, SnekObject>,

    snek_head_pos: SnekPosition,
    snek_head_dir: SnekDirection,
    snek_segments: Vec<SnekPosition>,
    snek_segments_pending: usize,
}

impl SnekGame {

    pub fn new(xsize: usize, ysize: usize) -> SnekGame {
        let snek_head_pos = SnekPosition { x: 10, y: 10, };
        let snek_head_dir = SnekDirection::East;
        let mut game = SnekGame {
                game_over: false,
                xsize,
                ysize,
                board: HashMap::new(),
                snek_head_pos,
                snek_head_dir,
                snek_segments: Vec::new(),
                snek_segments_pending: 3,
        };
        game.set_cell(&snek_head_pos, SnekObject::Head);
        // for i in 0..snek_length {
            // let seg_x = snek_head_pos.0 - 1 - i as i32;
            // game.set_cell((snek_head_pos.0 - 1 - i as i32) as usize, snek_head_pos.1 as usize, SnekObject::Segment);
            // self.snek_seg_pos.push(())
        // }
        
        let berry_pos = SnekPosition { x: 14, y: 10, };
        game.set_cell(&berry_pos, SnekObject::Berry);
        let rock_pos = SnekPosition { x: 20, y: 10, };
        game.set_cell(&rock_pos, SnekObject::Rock);
        game
    }

    /* Add a berry in a random, empty cell. */
    fn add_berry(&mut self) {
        self.add_random_object(SnekObject::Berry);
    }

    /* Add a rock in a random, empty cell. */
    fn add_rock(&mut self) {
        self.add_random_object(SnekObject::Rock);
    }

    fn add_random_object(&mut self, obj: SnekObject) {
        let mut rng = rand::thread_rng();
        loop {
            let x: i32 = rng.gen_range(0..self.xsize).try_into().unwrap();
            let y: i32 = rng.gen_range(0..self.ysize).try_into().unwrap();
            let berry_pos: SnekPosition = SnekPosition { x, y };
            if self.get_cell(&berry_pos) == SnekObject::Empty {
                self.set_cell(&berry_pos, obj);
                break;
            }
        }
    }

    /* Set a cell on the gameboard to a particular object.  If
     * the cell is outside the bounds of the board, this is a no-op,
     * although should possibly be an error?
     */
    fn set_cell(&mut self, pos: &SnekPosition, obj: SnekObject) {
        /* Sanity check on bounds. */
        if pos.x >= self.xsize as i32 || pos.y >= self.ysize as i32 {
            return;
        }

        /* If the cell was already occupied, clear it. */
        if self.get_cell(pos) != SnekObject::Empty {
            self.board.remove(pos);
        }

        /* Populate the cell with the specified SnekObject. */
        self.board.insert(*pos, obj);
    }

    /* Get the contents of a cell on the gameboard.  If the
     * cell is outside the gameboard, it is treated as a wall.
     * The x and y coordinates are passed as signed instead of
     * usize to make it easy to iterate over adjacent cells
     * which might land outside of the board.
     */
    fn get_cell(&self, pos: &SnekPosition) -> SnekObject {
        let ysize: i32 = self.ysize.try_into().unwrap();
        let xsize: i32 = self.xsize.try_into().unwrap();
        if pos.x < 0 || pos.x >= xsize || pos.y < 0 || pos.y > ysize {
            return SnekObject::Wall;
        }
        // let x: usize = x.try_into().unwrap();
        // let y: usize = y.try_into().unwrap();
        if let Some(cell) = self.board.get(pos) {
            cell.clone()
        } else {
            SnekObject::Empty
        }
    }

    /* Updates Snek's direction.  If you try to turn back on yourself,
     * it becomes a no-op.
     */
    // FIXME: 2022/01/11 - jbradach - this doesn't quite cover everything.  If I hit a full loop
    // FIXME: faster than a "tick" I can crash into myself in the same direction that gets drawn.
    // FIXME: ...do I need to delay a frame?  Or make it draw one more past game over...
    pub fn set_snekdir(&mut self, dir: SnekDirection) {
        let curdir = &self.snek_head_dir;
        match curdir {
            SnekDirection::North => { if dir == SnekDirection::South { return; } },
            SnekDirection::East => { if dir == SnekDirection::West { return; } },
            SnekDirection::West => { if dir == SnekDirection::East { return; } },
            SnekDirection::South => { if dir == SnekDirection::North { return; } },
        }
        self.snek_head_dir = dir;
    }

    // Called every game step
    pub fn step(&mut self) {
        // Check if game is in end state.  No-op if true.
        if self.game_over {
            return;
        }

        // Snek moves one step in facing direction
        let pos = self.snek_head_pos;
        let mut new_pos = pos;
        match self.snek_head_dir {
            SnekDirection::North => {
                new_pos.y -= 1;
            }
            SnekDirection::East => {
                new_pos.x += 1;
            }
            SnekDirection::West => {
                new_pos.x -= 1;
            }
            SnekDirection::South => {
                new_pos.y += 1;
            }
        }

        match self.get_cell(&new_pos) {
            SnekObject::Berry => {
                self.snek_segments_pending += 1;
                self.add_berry();
                self.add_rock();
            },
            SnekObject::Wall => {
                self.game_over = true;
                return;
            },
            SnekObject::Rock => {
                self.game_over = true;
                return;
            },
            SnekObject::Segment => {
                self.game_over = true;
                return;
            },
            _ => { /* WARK! */ },
        }

        /* Move the head on the board and update the head position. */
        self.board.remove(&pos);
        self.set_cell(&new_pos, SnekObject::Head);
        self.snek_head_pos = new_pos;

        /* Clear last round's segments from the board. */
        for i in 0..self.snek_segments.len() {
            self.board.remove(&self.snek_segments[i].clone());
        }

        /* Update the segment positions.  If we have segments waiting to be appended to Snek,
         * insert a new one at current head (x,y).  If head element (x,y) == head (x,y), don't
         * draw it and don't pop tail. */
        if self.snek_segments_pending > 0 {
            self.snek_segments.insert(0, pos);
            self.snek_segments_pending -= 1;
        }

        if self.snek_segments[0] != pos {
            self.snek_segments.insert(0, pos);
            self.snek_segments.pop();
        }
        
        for i in 0..self.snek_segments.len() {
            self.set_cell(&self.snek_segments[i].clone(), SnekObject::Segment);
        }
    }

    // Need a translate snakepos on the board to xy on the canvas.
    fn translate_snakepos_to_pos(&self, _canvas: &Canvas<Window>) -> (usize, usize) {
        (0, 0)
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.draw_board(canvas);
        self.draw_head(canvas);
        self.draw_segments(canvas);
        self.draw_rocks(canvas);
        self.draw_berries(canvas);
        // self.draw_grid(canvas);
    }

    fn draw_board(&self, canvas: &mut Canvas<Window>) {
        let (max_x, max_y) = canvas.output_size().unwrap();
        let BORDER_COLOR = Color::RGB(0, 200, 0);
        let board_max_x = 3 * (max_x / 4);
        let board_max_y = max_y;
        // Draw line around full canvas, 3-5 pixel thickness.  Rounded borders.
        // Draw line at 3/4ths (2/3rds or other fraction?) vertically
        // Left is gameboard, right is "game status"
        // Add "Snek!" text to status
        // Maybe animate it?
        // It should be a TTF font, C64?
        let (x1, y1) = (0, 0);
        let (x2, y2) = (board_max_x, board_max_y);
        
        // self.draw_line(&self, (x1, y1), (x2, y2));
        canvas.set_draw_color(BORDER_COLOR);
        canvas.set_draw_color(
            Color {
                r: BORDER_COLOR.r / 2,
                g: BORDER_COLOR.g / 2,
                b: BORDER_COLOR.b / 2,
                a: BORDER_COLOR.a,
            }
        );
        



        println!("{}, {}", board_max_x, board_max_y);
        
    }

    fn draw_head(&self, canvas: &mut Canvas<Window>) {
        const HEAD_COLOR: Color = Color::RGB(0, 80, 80);
        const FACE_COLOR: Color = Color::RGB(255, 80, 80);
        const FACE_WIDTH: u32 = 4;
        let pos = self.snek_head_pos;
        let x = pos.x * 32;
        let y = pos.y * 32;
        let orig_color = canvas.draw_color();
        canvas.set_draw_color(HEAD_COLOR);
        let _ = canvas.fill_rect(Rect::new(x, y, 32, 32));
        canvas.set_draw_color(FACE_COLOR);
        // match self.snek_head_dir {
            // SnekDirection::North => { canvas.fill_rect(Rect::new(x, y, 32, FACE_WIDTH)); },
            // SnekDirection::East => { canvas.fill_rect(Rect::new(x+32 - FACE_WIDTH, y, FACE_WIDTH, 32)); },
            // SnekDirection::West => { canvas.fill_rect(Rect::new(x, y, 32, 2)); },
            // SnekDirection::South => { canvas.fill_rect(Rect::new(x, y, 32, 1)); },
        // }

        canvas.set_draw_color(orig_color);
    }

    fn draw_segments(&self, canvas: &mut Canvas<Window>) {
        const SEGMENT_COLOR: Color = Color::RGB(255, 255, 0);
        let mut segments = self.board.clone();
        segments.retain(|_, v| v.clone() == SnekObject::Segment);
        let orig_color = canvas.draw_color();
        canvas.set_draw_color(SEGMENT_COLOR);
        for pos in segments.iter() {
            let x = pos.0.x * 32;
            let y = pos.0.y * 32;
            let _ =canvas.fill_rect(Rect::new(x, y, 32, 32));
        }
        canvas.set_draw_color(orig_color);
    }

    fn draw_berries(&self, canvas: &mut Canvas<Window>) {
        const BERRY_COLOR: Color = Color::RGB(255, 0, 0);
        let mut berries = self.board.clone();
        berries.retain(|_, v| v.clone() == SnekObject::Berry);
        let orig_color = canvas.draw_color();
        canvas.set_draw_color(BERRY_COLOR);
        for pos in berries.iter() {
            let x = pos.0.x * 32;
            let y = pos.0.y * 32;
            let _ = canvas.fill_rect(Rect::new(x, y, 32, 32));
        }
        canvas.set_draw_color(orig_color);
    }

    fn draw_rocks(&self, canvas: &mut Canvas<Window>) {
        const ROCK_COLOR: Color = Color::RGB(120, 120, 120);
        let mut rocks = self.board.clone();
        rocks.retain(|_, v| v.clone() == SnekObject::Rock);
        let orig_color = canvas.draw_color();
        canvas.set_draw_color(ROCK_COLOR);
        for pos in rocks.iter() {
            let x = pos.0.x * 32;
            let y = pos.0.y * 32;
            let _ = canvas.fill_rect(Rect::new(x, y, 32, 32));
        }
        canvas.set_draw_color(orig_color);
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
                let pos: SnekPosition = SnekPosition { x: x as i32, y: y as i32, };
                let obj = self.get_cell(&pos);
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
            writeln!(f)?;
        }
        writeln!(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
   
    fn do_game_steps(game: &mut SnekGame, ticks: usize) {
        for _ in 0..ticks {
            game.step();
            println!("{:?}", game);
            println!("{}", game);
            if game.game_over {
                return;
            }
            ::std::thread::sleep(Duration::new(0, 100_000_000)); // 1 second delay for debug
        }
    }
    
    #[test]
    fn test_snek_hit_snek() {
        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::North);
        do_game_steps(&mut game, 2);
        game.set_snekdir(SnekDirection::East);
        do_game_steps(&mut game, 2);
        game.set_snekdir(SnekDirection::South);
        do_game_steps(&mut game, 2);
        game.set_snekdir(SnekDirection::West);
        do_game_steps(&mut game, 2);
        assert_eq!(game.game_over, true);
    }

    #[test]
    fn test_snek_hit_wall() {
        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::North);
        do_game_steps(&mut game, 24);
        assert_eq!(game.game_over, true);

        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::East);
        do_game_steps(&mut game, 32);
        assert_eq!(game.game_over, true);

        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::West);
        do_game_steps(&mut game, 32);

        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::South);
        do_game_steps(&mut game, 24);
    }

    #[test]
    fn test_snek_diagonal() {
        let mut game = SnekGame::new(32, 24);
        game.set_snekdir(SnekDirection::North);
        do_game_steps(&mut game, 1);
        game.set_snekdir(SnekDirection::East);
        do_game_steps(&mut game, 1);
        game.set_snekdir(SnekDirection::South);
        do_game_steps(&mut game, 2);
        game.set_snekdir(SnekDirection::West);
        do_game_steps(&mut game, 32);
        assert_eq!(game.game_over, true);

    }

}