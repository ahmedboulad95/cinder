use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;
use sdl2::EventPump;
use sdl2::render::{WindowCanvas};
use crate::cinder_core::ecs::Entity;
use crate::cinder_core::media::renderer;
use crate::cinder_core::Direction;

const PLAYER_MOVEMENT_SPEED: i32 = 10;

pub struct Game {
    player: Entity,
    event_pump: EventPump,
    canvas: WindowCanvas,
    i: u8
}

impl Game {
    pub fn init() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        let window = video_subsystem
            .window("Game", 800, 600)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Could not make a canvas");

        Game {
            player: Entity {
                position: Point::new(0, 0),
                sprite: Rect::new(0, 0, 64, 64),
                texture_name: String::from("assets/boy.png"),
                speed: 0,
                direction: Direction::Down
            },
            event_pump: sdl_context.event_pump().unwrap(),
            canvas: canvas,
            i: 0
        }
    }

    pub fn run(&mut self) {
        loop {
            let has_quit = self.handle_events();
            if has_quit { break; }
            self.update();
            self.render();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.player.speed = PLAYER_MOVEMENT_SPEED;
                    self.player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.player.speed = PLAYER_MOVEMENT_SPEED;
                    self.player.direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.player.speed = PLAYER_MOVEMENT_SPEED;
                    self.player.direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.player.speed = PLAYER_MOVEMENT_SPEED;
                    self.player.direction = Direction::Down;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    self.player.speed = 0;
                },
                _ => {}
            }
        }
        false
    }

    fn update(&mut self) {
        self.i = (self.i + 1) % 255;
        self.player.update();
    }

    fn render(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/boy.png").unwrap();
        renderer::render(&mut self.canvas, Color::RGB(255, 255, 255), &texture, &self.player);
    }
}