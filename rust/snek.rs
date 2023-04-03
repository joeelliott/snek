// SNEK
// Cold Open, go!
extern crate piston_window;
    
use piston_window::*;
use std::collections::LinkedList;
    
const BLOCK_SIZE: f64 = 25.0;
    
#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    }
    
#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
    }
    
pub struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    }
    
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
    
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block { x, y });
    
        Snake {
            body,
            direction: Direction::Right,
        }
    }
    
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }
    
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => {}
        }
    
        let (last_x, last_y) = self.head_position();
    
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
    
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
    }
    
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
    }
    
fn draw_block(x: i32, y: i32, color: [f32; 4], g: &mut G2d) {
    let gui_x = (x as f64) * BLOCK_SIZE;
    let gui_y = (y as f64) * BLOCK_SIZE;
    
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        g.transform,
        g,
    );
    }
    
fn draw_snake(snake: &Snake, g: &mut G2d) {
    for block in &snake.body {
        draw_block(block.x, block.y, [0.0, 1.0, 0.0, 1.0], g);
    }
    }
    
fn main() {
    let (width, height) = (800, 600);
    
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut snake = Snake::new(5, 5);
    
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key
            Key::Up => snake.move_forward(Some(Direction::Up)),
            Key::Down => snake.move_forward(Some(Direction::Down)),
            Key::Left => snake.move_forward(Some(Direction::Left)),
            Key::Right => snake.move_forward(Some(Direction::Right)),
            _ => {}
        }
    }
    
    window.draw_2d(&event, |c, g, _| {
        clear([1.0; 4], g);
        draw_snake(&snake, g);
    });
    
    if let Some(u) = event.update_args() {
        snake.move_forward(None);
    }
    }
// and 'scene'
