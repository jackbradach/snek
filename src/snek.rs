#![allow(dead_code)]


use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use colored::{Colorize};

use sdl2::pixels::{Color};
use sdl2::rect::Point;
use sdl2::render::{Canvas};
// use sdl2::surface::Surface;
use sdl2::video::{Window};

// TODO; have a struct for points in the game board.  Replace all x, y function calls.

#[derive(Debug)]
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
                snek_segments_pending: 6,
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

    fn add_berry(&mut self, _pos: SnekPosition) {

    }

    fn add_rock(&mut self, _pos: SnekPosition) {

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

    pub fn set_snekdir(&mut self, dir: SnekDirection) {
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
                println!("Snek ate a berry @ ({}, {})!", new_pos.x, new_pos.y);
                self.snek_segments_pending += 1;
            },
            SnekObject::Wall => {
                println!("Snek hit the wall @ ({}, {})!", new_pos.x, new_pos.y);
                self.game_over = true;
                return;
            },
            SnekObject::Rock => {
                println!("Snek hit a rock @ ({}, {})!", new_pos.x, new_pos.y);
                self.game_over = true;
                return;
            },
            SnekObject::Segment => {
                println!("Snek hit Snek @ ({}, {})!", new_pos.x, new_pos.y);
                self.game_over = true;
                return;
            },
            _ => { /* WARK! */ },
        }

        /* Move the head on the board and update the head position. */
        self.board.remove(&pos);
        // NOTE: stopped SnakePosition refactor here.
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

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.draw_grid(canvas);
        self.draw_head(canvas);
    }

    fn draw_head(&self, _canvas: &mut Canvas<Window>) {

    }

    fn draw_segments(&self, _canvas: &mut Canvas<Window>) {
        // iterate over segment vector, drawing each in
        // alternating colors.  
    }

    fn draw_berries(&self, _canvas: &mut Canvas<Window>) {

    }

    fn draw_rocks(&self, _canvas: &mut Canvas<Window>) {

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