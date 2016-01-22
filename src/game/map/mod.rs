extern crate rand;
use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};

use super::object;


pub enum FloorType {
  Lava,
  Ground,
  Pit,
  Wall,
}

pub struct Tile<'a> {
  pub floor: FloorType,
  //object: &Object,
  symbol: char,
  occupied: bool,
  pc: Option<&'a object::PlayerCharacter<'a>>,
}

impl<'a> Tile<'a> {
  fn new() -> Tile<'a> {
    Tile {floor: FloorType::Wall, symbol: '=', occupied: true, pc: None}
  }
}

pub struct Shape {

}

pub struct Room<'a> {
  my_index: usize,
  left: Option<usize>,
  right: Option<usize>,
  down: Option<usize>,
  up: Option<usize>,
  shape: Shape,

}

pub struct Map<'a> {
  pub tilemap: Vec<Vec<Tile<'a>>>,
  pub bounds: object::Point<usize>,
  pub rooms: Vec<Room<'a>>,
  pub room_tmpls: Vec<Vec<Vec<FloorType>>>,
  pub start: object::Point<usize>,
}

impl<'a> Map<'a> {

  pub fn new(bnds: object::Point<usize>) -> Map<'a> {
    Map {
      tilemap: (0..25).map(|_| (0..25).map(|_| Tile::new()).collect()).collect(),
      bounds: bnds,
      rooms: vec![],
      room_tmpls: vec![],
      start: Point {x: 0, y: 0},
    }
  }

  pub fn is_occupied(&mut self, loc: object::Point<u16>) -> bool {
    self.tilemap[loc.x as usize][loc.y as usize].occupied
  }

  pub fn move_object(&mut self, orig: object::Point<u16>, dest: object::Point<u16>) {
    if (self.tilemap[dest.x as usize][dest.y as usize].pc.is_none()) && (self.tilemap[orig.x as usize][orig.y as usize].pc.is_some()) {
      self.tilemap[dest.x as usize][dest.y as usize].pc = self.tilemap[orig.x as usize][orig.y as usize].pc;
      self.tilemap[orig.x as usize][orig.y as usize].pc = None;
    }

  }

  fn generate_room_tmpl(&mut self) {
    self.room_tmpls.push(vec![
      vec![FloorType::Ground, FloorType::Ground, FloorType::Ground],
      vec![FloorType::Ground, FloorType::Ground, FloorType::Ground],
      vec![FloorType::Ground, FloorType::Ground, FloorType::Ground]
    ]);
    self.room_tmpls.push(vec![
      vec![FloorType::Ground, FloorType::Ground, FloorType::Ground]
    ]);

    self.room_tmpls.push(vec![
      vec![FloorType::Ground],
      vec![FloorType::Ground],
      vec![FloorType::Ground]
    ]);

    self.room_tmpls.push(vec![
      vec![FloorType::Ground]
    ]);

  }

  pub fn generate_walls(&mut self) {
    let mut rng = rand::thread_rng();
    let range = Range::new(0, 4);
    let strt_range = Range::new(0usize, 25);
    for x in 0..25 {
      for y in 0..25 {
        let rnd_num = range.ind_sample(&mut rng);
        if x == 0 || y == 0 || x == 24 || y == 24 {
          self.tilemap[x][y] = Tile {floor: FloorType::Wall, symbol: '=', occupied: true, pc: None};
        }
        else {
          if rnd_num > 1 {
            self.tilemap[x][y] = Tile {floor: FloorType::Ground, symbol: '.', occupied: false, pc: None};
          }
        }
      }
    }
    let mut no_start_point = true;

    // Find a good starting point
    while no_start_point {

      let start_x = strt_range.ind_sample(&mut rng);
      let start_y = strt_range.ind_sample(&mut rng);

      if self.tilemap[start_x][start_y].floor == FloorType::Ground {
        self.start = Point {x: start_x, y: start_y};
        no_start_point = false;
      }
    }
  }
}
