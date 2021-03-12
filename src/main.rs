use crate::cinder_core::game::Game;

pub mod cinder_core;

fn main() -> Result<(), String> {
    let mut game = Game::init();
    game.run();
    Ok(())
}