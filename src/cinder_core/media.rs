pub mod renderer {
    use sdl2::pixels::Color;
    use sdl2::render::{WindowCanvas, Texture};
    use sdl2::rect::{Point, Rect};
    use crate::cinder_core::ecs::Entity;

    pub fn render(
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
}