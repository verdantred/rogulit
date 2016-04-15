extern crate rand;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::ThreadRng;
use std::collections::HashMap;

use super::object;

#[derive(PartialEq, Debug)]
pub enum FloorType {
  Lava,
  Ground,
  Pit,
  Wall,
}

#[derive(PartialEq, Debug)]
pub enum RoomType {
  Rectangular,
  Round,
  Hallway,
}

pub struct Tile<'a> {
  pub floor: FloorType,
  //object: &Object,
  occupied: bool,
  pub pc: Option<&'a str>,
}

impl<'a> Tile<'a> {
  fn new() -> Tile<'a> {
    Tile {floor: FloorType::Wall, occupied: true, pc: None}
  }
}

pub struct Room<'a> {
  id: u16,
  tiles: Vec<Vec<Tile<'a>>>,
  r_type: RoomType,
}

pub struct Map<'a> {
  pub tilemap: Vec<Vec<Tile<'a>>>,
  pub bounds: object::Point<usize>,
  //pub rooms: Vec<&'a Room<'a>>,
  room_count: u32,
  pub start: object::Point<usize>,
}

impl<'a> Map<'a> {

  pub fn new(bnds: object::Point<usize>) -> Map<'a> {
    Map {
      tilemap: (0..bnds.x).map(|_| (0..bnds.y).map(|_| Tile::new()).collect()).collect(),
      bounds: bnds,
      room_count: 0,
      start: object::Point {x: 0, y: 0},
    }
  }
  pub fn get_room_count(&mut self) -> u32 {
    self.room_count
  }
  pub fn is_occupied(&mut self, loc: object::Point<usize>) -> bool {
    self.tilemap[loc.x][loc.y].occupied
  }

  pub fn is_within_bounds(&mut self, loc: object::Point<usize>) -> bool {
    if (loc.x < self.bounds.x - 1) && (loc.x >= 1) && (loc.y < self.bounds.y - 1) && (loc.y >= 1) {
      println!("within");
      return true;
    }
    return false;
  }

  pub fn move_object(&mut self, orig: object::Point<usize>, dest: object::Point<usize>) -> bool {
    println!("{:?} ja {:?}", orig, dest);
    if self.is_within_bounds(dest) && !self.is_occupied(dest) && (self.tilemap[dest.x][dest.y].pc.is_none()) && (self.tilemap[orig.x][orig.y].pc.is_some()) {
      self.tilemap[dest.x][dest.y].pc = self.tilemap[orig.x][orig.y].pc;
      self.tilemap[orig.x][orig.y].pc = None;
      println!("voi siirtää");
      return true;
    }
    return false;

  }

  pub fn check_space(&mut self, start: object::Point<usize>, end: object::Point<usize>) -> bool {
    for x in start.x..(end.x + 1) {
      for y in start.y..(end.y + 1) {
        if self.tilemap[x][y].floor == FloorType::Ground {
          return false;
        }
      }
    }
    return true;
  }

  pub fn get_new_doorways(&mut self, start: object::Point<usize>, end: object::Point<usize>) -> Vec<object::Point<usize>>{
    let mut door_vector = Vec::new();
    for x in start.x..(end.x + 1) {
      println!("HMMMMMM");
      if end.y + 1 < self.bounds.y - 1 {
        door_vector.push(object::Point {x: x, y: end.y + 1})
      }
      if start.y - 1 >= 1 {
        door_vector.push(object::Point {x: x, y: start.y - 1})
      }
    }
    for y in start.y..(end.y + 1) {
      if end.x + 1 < self.bounds.x - 1 {
        door_vector.push(object::Point {x: end.x + 1, y: y})
      }
      if start.x - 1 >= 1 {
        door_vector.push(object::Point {x: start.x - 1, y: y})
      }
    }
    door_vector
  }

  pub fn generate_rooms(&mut self, percentile: f32) {
    let bnds = self.bounds;
    println!("percentile: {}", percentile);

    let mut rng = rand::thread_rng();
    let width_range = Range::new(3usize, self.bounds.x - 4);
    let height_range = Range::new(3usize, self.bounds.y - 4);

    if percentile == 1f32 {
      self.fill_rect(object::Point {x: 1, y: 1}, object::Point {x: bnds.x - 2, y: bnds.y - 2});
      self.generate_starting_point(&width_range, &height_range);
      return;
    }

    let mut realized = 0f32;
    let space = (self.bounds.x - 2) * (self.bounds.y - 2);
    let mut corridor = false;

    let mut available_doorways: Vec<object::Point<usize>> = Vec::new();
    let start = object::Point {x: width_range.ind_sample(&mut rng), y: height_range.ind_sample(&mut rng)};


    self.fill_rect(start, start);
    let new_doors = self.get_new_doorways(start, start);
    for door in new_doors {
      println!("({},{})", door.x, door.y);
      available_doorways.push(door);
    }

    let mut j = 0;
    while (j < available_doorways.len()) && (percentile > (realized + 0.01)){
      j += 1;
      println!("-------------------------");
      println!("realized: {}", realized);

      for door in &available_doorways {
        println!("({},{})", door.x, door.y);
      }

      let mut dir = object::Direction::Still;
      println!("length {}", available_doorways.len());
      let door_range = Range::new(0usize, available_doorways.len());
      let indx = door_range.ind_sample(&mut rng);
      println!("index: {}", indx);
      let loc = object::Point {x: available_doorways[indx].x, y: available_doorways[indx].y};
      println!("start: {} {}", start.x, start.y);
      println!("new: {} {}", loc.x, loc.y);

      if self.tilemap[loc.x + 1][loc.y].floor == FloorType::Ground {
        dir = object::Direction::Left;
      }
      if self.tilemap[loc.x - 1][loc.y].floor == FloorType::Ground {
        dir = object::Direction::Right;
      }
      if self.tilemap[loc.x][loc.y + 1].floor == FloorType::Ground {
        dir = object::Direction::Down;
      }
      if self.tilemap[loc.x][loc.y - 1].floor == FloorType::Ground {
        dir = object::Direction::Up;
      }

      if dir == object::Direction::Still {
        println!("loc: {}x{}, start: {}x{}", loc.x, loc.y, start.x, start.y);
        continue;
      }


      let mut size = object::Point {x: ((width_range.ind_sample(&mut rng) as f32) * (percentile - realized)) as usize,
                                y: ((height_range.ind_sample(&mut rng) as f32) * (percentile - realized )) as usize};
      if size.x < 3 {
        size.x = 3;
      }
      if size.y < 3 {
        size.y = 3;
      }
      println!("size: {} x {}", size.x, size.y);
      if corridor {
        if (dir ==  object::Direction::Up) || (dir ==  object::Direction::Down) {
          size.x = 3;
        }
        else {size.y = 3;}
      }

      match self.does_room_fit(loc, size, dir) {
        Some((r_start, r_end)) => {
          corridor = !corridor;
          realized += (size.x * size.y) as f32 / space as f32;
          self.fill_rect(loc, loc);
          self.fill_rect(r_start, r_end);
          available_doorways.append(&mut self.get_new_doorways(r_start, r_end));
          available_doorways.iter().position(|&n| n.x == loc.x && n.y == loc.y).map(|e| available_doorways.remove(e));
          self.room_count += 1;
        }
        None => {
          continue;
        }
      }

    }
    //self.generate_triggers();

    self.generate_starting_point(&width_range, &height_range);

    //self.populate_with_monsters();
    //self.generate_items();

  }

  pub fn does_room_fit(&mut self, loc: object::Point<usize>, size: object::Point<usize>, dir: object::Direction) -> Option<(object::Point<usize>, object::Point<usize>)> {

    let mut start = object::Point {x: 1, y: 1};
    let mut end = object::Point {x: 1, y: 1};

    match dir {

      object::Direction::Up => {
        if (loc.x as i32) - ((size.x / 2) as i32) < 1 {
          return None;
        }
        if (loc.y + 1) >= self.bounds.y - 1 {
          return None;
        }

        start.x = loc.x - (size.x / 2);
        start.y = loc.y + 1;

        if (start.x + size.x - 1) >= self.bounds.x - 1 {
          return None;
        }
        if (start.y + size.y - 1) >= self.bounds.y - 1 {
          return None;
        }
        end.x = start.x + size.x - 1;
        end.y = start.y + size.y - 1;
        println!("ylös-hyvä-huone");
      }
      object::Direction::Down => {
        if (loc.x + (size.x / 2)) >= (self.bounds.x - 1) {
          return None;
        }
        if (loc.y - 1) < 1 {
          return None;
        }

        end.x = loc.x + (size.x / 2);
        end.y = loc.y - 1;

        if ((end.x as i32) - ((size.x + 1) as i32)) < 1 {
          return None;
        }
        if ((end.y as i32) - ((size.y + 1) as i32)) < 1 {
          return None;
        }
        start.x = end.x - size.x + 1;
        start.y = end.y - size.y + 1;
        println!("alas-hyvä-huone");
      }
      object::Direction::Left => {
        if (loc.x - 1) < 1 {
          return None;
        }
        if (loc.y + (size.y / 2)) >= self.bounds.y - 1 {
          return None;
        }
        end.x = loc.x - 1;
        end.y = loc.y + (size.y / 2);

        if ((end.x as i32) - (size.x as i32) + 1) < 1{
          return None;
        }
        if ((end.y as i32) - (size.y as i32) + 1) < 1{
          return None;
        }
        start.x = end.x - size.x + 1;
        start.y = end.y - size.y + 1;
        println!("vas-hyvä-huone");
      }
      object::Direction::Right => {
        if (loc.x + 1) < 1 {
          return None;
        }
        if ((loc.y as i32) - ((size.y as i32) / 2)) < 1 {
          return None;
        }
        start.x = loc.x + 1;
        start.y = loc.y - (size.y / 2);

        if (start.x + size.x - 1) >= self.bounds.x - 1 {
          return None;
        }
        if (start.y + size.y - 1) >= self.bounds.y - 1 {
          return None;
        }
        end.x = start.x + size.x - 1;
        end.y = start.y + size.y - 1;
        println!("oik-hyvä-huone");
      }
      _ => {
        println!("EI TÄNNE");
        return None;
      }

    }
    println!("start: ({},{}), end: ({},{})", start.x, start.y, end.x, end.y);
    let result = self.check_space(object::Point {x: start.x - 1, y: start.y - 1},
                                  object::Point {x: end.x + 1, y: end.y + 1});
    println!("result: {}", result);
    if !(result){
      return None;
    }
    return Some((start, end));
  }

  pub fn generate_starting_point(&mut self, w_range: &Range<usize>, h_range: &Range<usize>) {
    let mut no_start_point = true;
    let mut rng = rand::thread_rng();
    // Find a good starting point
    while no_start_point {

      let start_x: usize = w_range.ind_sample(&mut rng);
      let start_y: usize = h_range.ind_sample(&mut rng);

      if self.tilemap[start_x][start_y].floor == FloorType::Ground {
        self.start = object::Point {x: start_x, y: start_y};
        no_start_point = false;
      }
    }
  }

  pub fn fill_rect(&mut self, start: object::Point<usize>, end: object::Point<usize>) {
    for x in start.x..(end.x + 1) {
      for y in start.y..(end.y + 1) {
        self.tilemap[x][y].floor = FloorType::Ground;
        self.tilemap[x][y].occupied = false;
      }
    }
  }


}
