use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;

#[derive(Debug)]
struct Entity {
    position: Point,
    sprite: Rect
}

fn render(
    canvas: &mut WindowCanvas, 
    color: Color, 
    texture: &Texture, 
    entity: &Entity
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = entity.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, entity.sprite.width(), entity.sprite.height());
    canvas.copy(texture, entity.sprite, screen_rect)?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Game", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/boy.png")?;

    let player = Entity {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 64, 64)
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}