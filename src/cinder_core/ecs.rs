use sdl2::rect::{Point, Rect};
use crate::cinder_core::Direction;

#[derive(Debug)]
pub struct Entity {
    pub position: Point,
    pub sprite: Rect,
    pub texture_name: String,
    pub speed: i32,
    pub direction: Direction
}

impl Entity {
    pub fn update(&mut self) {
        match self.direction {
            Direction::Left => self.position = self.position.offset(-self.speed, 0),
            Direction::Right => self.position = self.position.offset(self.speed, 0),
            Direction::Up => self.position = self.position.offset(0, -self.speed),
            Direction::Down => self.position = self.position.offset(0, self.speed)
        }
    }
}