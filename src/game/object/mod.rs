use std::cell::RefCell;
use super::map::Map;

struct Item<'a> {
  id: u32,
  name: &'a str,
}

struct Inventory<'a> {
  items: Vec<Item<'a>>,
  money: u32,
}

impl<'a> Inventory<'a> {
  fn new() -> Inventory<'a> {
    Inventory {items: vec![], money: 0}
  }
}

#[derive(Copy, Clone)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

pub struct Movement {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
}

impl Movement {
  pub fn new() -> Movement {
    Movement {up: false, down: false, left: false, right: false}
  }
}

pub enum Direction {
  Up, Down, Left, Right, Still,
}

pub struct PlayerCharacter<'a> {
  inven: Inventory<'a>,
  health: u32,
  delta_mov: Point<f64>,
  speed: f64,
  strength: u32,
  defense: u32,
  pub loc: Point<u16>,
  pub mov_dir: Movement,
  movable: bool,
  map: &'a RefCell<Map<'a>>,
}

impl<'a> PlayerCharacter<'a> {

  pub fn new(map: &'a RefCell<Map<'a>>) -> PlayerCharacter<'a> {
    PlayerCharacter {inven: Inventory::new(), health: 5, speed: 0.12f64, strength: 5, defense: 5,
                    loc: map.borrow_mut().start, mov_dir: Movement::new(), movable: true, map: map,
                    delta_mov: Point {x: 0f64, y: 0f64}}
  }

  pub fn move_pc(&mut self) {
    let mut new_loc = self.loc;

    let mut delta = Point {x: self.delta_mov.x, y: self.delta_mov.y};

    if self.mov_dir.up {
      delta.y -= self.speed;
    }
    if self.mov_dir.left {
      delta.x -= self.speed;
    }
    if self.mov_dir.down {
      delta.y += self.speed;
    }
    if self.mov_dir.right {
      delta.x += self.speed;
    }
    if !self.mov_dir.down && !self.mov_dir.up {delta.y = 0f64;}
    if !self.mov_dir.right && !self.mov_dir.left {delta.x = 0f64;}

    if (delta.x < 1f64) && (delta.x > -1f64) {
      self.delta_mov.x = delta.x;
    }
    else {
      self.delta_mov.x = delta.x.fract();
      let testx = new_loc.x as f64 + delta.x.trunc();
      if testx < 0f64 {
        new_loc.x = 0u16;
      }
      else if testx > 24f64 {
        new_loc.x = 24u16;
      }
      else {new_loc.x = testx as u16}

    }
    if (delta.y < 1f64) && (delta.y > -1f64) {
      self.delta_mov.y = delta.y;
    }
    else {
      self.delta_mov.y = delta.y.fract();
      let testy = new_loc.y as f64 + delta.y.trunc();
      if testy < 0f64 {
        new_loc.y = 0u16;
      }
      else if testy > 24f64 {
        new_loc.y = 24u16;
      }
      else {new_loc.y = testy as u16}

    }
    //if self.mov_dir.up || self.mov_dir.left {
      //println!("{},{}", delta.x, delta.y);
    //}

    if ((new_loc.y != self.loc.y) || (new_loc.x != self.loc.x)) && (!self.map.borrow_mut().is_occupied(new_loc)){
      self.map.borrow_mut().move_object(self.loc, new_loc);
      self.loc = new_loc;
    }
  }

}
