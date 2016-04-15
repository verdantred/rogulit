extern crate rand;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::cell::RefCell;
use self::rand::distributions::{IndependentSample, Range};

use self::piston::input::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };

pub mod map;
pub mod object;

use self::map::Map;
use self::map::FloorType;
use self::object::*;

pub const BOUNDS: object::Point<usize> = object::Point {x: 75, y: 75};

pub struct Game<'a> {
  map: Vec<Vec<&'a RefCell<Map<'a>>>>,
  map_indx: Point<usize>,
  pc: PlayerCharacter<'a>,
  esc: bool,
  gl: GlGraphics,
  //monsters: Monsters,
}

impl<'a> Game<'a> {

  pub fn new(map: &'a RefCell<Map<'a>>) -> Game<'a> {

    let between = Range::new(0f32, 1f32);
    let mut rng = rand::thread_rng();
    let percentile = between.ind_sample(&mut rng);
    {
      let mut map_ref = map.borrow_mut();
      while map_ref.get_room_count() < 3 {
        map_ref.generate_rooms(percentile);
      }
    }

    Game {map: vec![vec![map]], map_indx: Point {x: 0, y: 0},
          pc: PlayerCharacter::new(map), esc: false, gl: GlGraphics::new(OpenGL::V3_2)}
  }

  pub fn init(& mut self) {
    let start = self.map[0][0].borrow_mut().start;
    self.map[0][0].borrow_mut().tilemap[start.x][start.y].pc = Some(self.pc.id);
  }

  pub fn check_input(&mut self, inp: Input) {
    match inp {
        Input::Press(but) => {
          match but {
            Button::Keyboard(Key::Up) => {
              self.pc.mov_dir.up = true;
            }
            Button::Keyboard(Key::Down) => {
              self.pc.mov_dir.down = true;
            }
            Button::Keyboard(Key::Left) => {
              self.pc.mov_dir.left = true;
            }
            Button::Keyboard(Key::Right) => {
              self.pc.mov_dir.right = true;
            }
            _ => {}
          }
        }

        Input::Release(but) => {
          match but {
            Button::Keyboard(Key::Up) => {
              self.pc.mov_dir.up = false;
            }
            Button::Keyboard(Key::Down) => {
              self.pc.mov_dir.down = false;
            }
            Button::Keyboard(Key::Left) => {
              self.pc.mov_dir.left = false;
            }
            Button::Keyboard(Key::Right) => {
              self.pc.mov_dir.right = false;
            }
            _ => {}
          }
        }
        _ => {}

      }
  }

  pub fn update(&mut self, upd: &UpdateArgs) {
    self.pc.move_pc();
  }

  pub fn render(&mut self, ren: &RenderArgs) {
        use self::graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);
        let map_ref = self.map[self.map_indx.x][self.map_indx.y].borrow();
        let (x, y) = (self.pc.loc.x as f64,
                      (self.pc.loc.y) as f64);


        self.gl.draw(ren.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            let transform_pc = c.transform.trans(x * 10f64, y * 10f64);
            for x_w in 0..map_ref.bounds.x {
              for y_w in 0..map_ref.bounds.y {
                match map_ref.tilemap[x_w][y_w].floor {
                  FloorType::Wall => rectangle(BLUE, square, c.transform.trans((x_w * 10) as f64, (y_w * 10) as f64), gl),
                  FloorType::Lava => rectangle(RED, square, c.transform.trans((x_w * 10) as f64, (y_w * 10) as f64), gl),
                  FloorType::Pit => rectangle(BLACK, square, c.transform.trans((x_w * 10) as f64, (y_w * 10) as f64), gl),
                  _ => {}
                }
              }
            }

            // Draw the player character
            ellipse(GREEN, square, transform_pc, gl);
            //rectangle(GREEN, square, transform_pc, gl);

        });
    }
}
