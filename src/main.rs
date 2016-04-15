extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::event_loop::*;
use self::piston::input::*;
use self::piston::window::WindowSettings;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::OpenGL;

use std::cell::RefCell;
mod game;
use game::Game;
use game::map::Map;

fn main() {
    println!("Hello, world!");
    let opengl = OpenGL::V3_2;
    let window: Window = WindowSettings::new(
            "rogulit",
            [(game::BOUNDS.x as u32) * 10 , (game::BOUNDS.y as u32) * 10]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let map = RefCell::new(Map::new(game::BOUNDS));

    let mut new_game = Game::new(&map);
    new_game.init();
    for e in window.events() {
      match e {
        Event::Update(upd) => {
          new_game.update(&upd);
        }
        Event::Render(ren) => {
          new_game.render(&ren);
        }
        Event::Input(inp) => {
          new_game.check_input(inp);
        }
        _ => {

        }
      }

    }
}
