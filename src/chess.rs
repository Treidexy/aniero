use std::{ops::{Index, IndexMut}, rc::Rc};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord {
	pub idx: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Side {
	pub idx: u8,
}

#[derive(Clone)]
pub struct TroopPlay {
	pub to: Coord,
	pub threats: Vec<Coord>,
}

#[derive(Clone)]
pub struct MovePly {
	pub troop: TroopId,
	pub from: Coord,
	pub to: Coord,
	pub kills: Vec<Coord>,
}

#[derive(Clone)]
pub struct CastlePly {
	pub from: Coord,
	pub to: Coord,
	pub castle_from: Coord,
	pub castle_to: Coord,
}

#[derive(Clone)]
pub enum Ply {
	None,
	Castle(CastlePly),
	Move(MovePly),
	Promote(MovePly, TroopId),
	Resign,
}

pub struct Game {
	pub width: u8,
	pub height: u8,
	pub troops: Vec<Troop>,
	pub multi_castle: bool,
}

// width * height <= 256
#[derive(Clone)]
pub struct Position {
	pub game: Rc<Game>,
	pub alphas: [Option<Coord>; 8],
	pub tiles: [Option<(Side, TroopId)>; 256],
	pub recent_plys: [Ply; 24],

	// first: start coord; last: end coord
	pub passant: Vec<Coord>,
	pub castlings: [Vec<Castling>; 8],
}

#[derive(Clone)]
pub struct Castling {
	castle_from: Coord,
	castle_to: Coord,

	passing: Vec<Coord>,
	castles: Vec<TroopId>,
}

impl Castling {
	fn can_castle(&self, position: &Position, side: Side) -> bool {
		let Some((piece_side, piece)) = position.tiles[self.castle_from] else {
			return false;
		};

		if piece_side != side {
			return false;
		}

		if !self.castles.contains(&piece) {
			return false;
		}

		for &coord in &self.passing {
			if position.tiles[coord].is_some() {
				return false;
			}
		}

		true
	}

	fn do_castle(&self, position: &mut Position) -> CastlePly {
		position.passant = self.passing.clone();
		let from = *self.passing.first().unwrap();
		let to = *self.passing.last().unwrap();

		let (side, piece) = position.tiles[from].unwrap();
		position.tiles[from] = None;
		position.tiles[to] = Some((side, piece));

		let (side, castle) = position.tiles[self.castle_from].unwrap();
		position.tiles[self.castle_from] = None;
		position.tiles[self.castle_to] = Some((side, castle));

		CastlePly { from, to, castle_from: self.castle_from, castle_to: self.castle_to }
	}
}

pub struct PositionInfo {
	pub threats: [[bool; 8]; 256],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TroopId {
	pub idx: u8,
}

pub struct TroopInfo {
	pub id: TroopId,
	pub coord: Coord,
	pub side: Side,
}

// more like TroopKind
pub struct Troop {
	pub char: char,
	pub get_plays: fn(position: &Position, info: TroopInfo) -> Vec<TroopPlay>,
	pub do_play: fn(position: &mut Position, info: TroopInfo, play: &TroopPlay) -> MovePly,
}

impl From<u8> for Side {
	fn from(idx: u8) -> Side {
		Side { idx: idx }
	}
}

impl From<i32> for Side {
	fn from(idx: i32) -> Side {
		Side { idx: idx as u8 }
	}
}

impl From<i32> for TroopId {
	fn from(idx: i32) -> TroopId {
		TroopId { idx: idx as u8 }
	}
}

impl From<usize> for Side {
	fn from(idx: usize) -> Side {
		Side { idx: idx as u8 }
	}
}

impl From<u8> for Coord {
	fn from(idx: u8) -> Coord {
		Coord { idx: idx }
	}
}

impl From<i32> for Coord {
	fn from(idx: i32) -> Coord {
		Coord { idx: idx as u8 }
	}
}

impl From<usize> for Coord {
	fn from(idx: usize) -> Coord {
		Coord { idx: idx as u8 }
	}
}

impl Coord {
	pub fn from_xy(game: &Game, x: u8, y: u8) -> Coord {
		Coord { idx: x + y * game.width }
	}

	pub fn decomp(self, game: &Game) -> (u8, u8) {
		(self.idx % game.width, self.idx / game.width)
	}

	pub fn fmt(self, game: &Game) -> String {
		let (x, y) = self.decomp(game);
		format!("{}{}", (x as u8 + b'a') as char, (y as u8 + b'1') as char)
	}
}

impl<T> Index<Coord> for [T] {
	type Output = T;

	fn index(&self, coord: Coord) -> &T {
		&self[coord.idx as usize]
	}
}

impl<T> IndexMut<Coord> for [T] {
	fn index_mut(&mut self, coord: Coord) -> &mut T {
		&mut self[coord.idx as usize]
	}
}

impl<T> Index<Side> for [T] {
	type Output = T;

	fn index(&self, side: Side) -> &T {
		&self[side.idx as usize]
	}
}

impl<T> IndexMut<Side> for [T] {
	fn index_mut(&mut self, side: Side) -> &mut T {
		&mut self[side.idx as usize]
	}
}

impl Game {
	pub fn get_troop(&self, id: TroopId) -> &Troop {
		&self.troops[id.idx as usize]
	}
}

impl Position {
	pub fn troop_plays(&self, coord: Coord) -> Vec<TroopPlay> {
		let Some((side, id)) = self.tiles[coord] else {
			return Vec::new();
		};

		let game = &self.game;
		let troop = game.get_troop(id);
		(troop.get_plays)(self, TroopInfo { id, coord, side })
	}

	pub fn troop_play(&mut self, coord: Coord, play: &TroopPlay) {
		let Some((side, id)) = self.tiles[coord] else {
			return;
		};

		let game = &self.game;
		let troop = game.get_troop(id);
		let ply = Ply::Move((troop.do_play)(self, TroopInfo { id, coord, side }, play));
		self.recent_plys[0] = ply;
	}

	pub fn analyze(&self) -> PositionInfo {
		let game = &self.game;
		let mut info = PositionInfo {
			threats: [[false; 8]; 256],
		};

		for idx in 0..game.width * game.height {
			let coord = Coord::from(idx);
			if let Some((side, id)) = self.tiles[coord] {
				let troop = game.get_troop(id);
				let plays = (troop.get_plays)(self, TroopInfo { id, coord, side });

				for play in plays {
					for threat in play.threats {
						info.threats[threat][side] = true;
					}
				}
			}
		}

		info
	}

	pub fn in_check(&self, side: Side) -> bool {
		let Some(alpha) = self.alphas[side] else {
			return false;
		};
	
		self.analyze().threats[alpha].iter().enumerate().filter(|(i, _)| Side::from(*i) != side).map(|(_, b)| *b).reduce(|a, b| a || b).unwrap()
	}

	pub fn is_play_safe(&self, coord: Coord, play: &TroopPlay) -> bool {
		let (side, troop_id) = self.tiles[coord].unwrap();
		let Some(alpha) = self.alphas[side] else {
			return true;
		};
	
		// clone the position and do the play
		let mut test_position = self.clone();
		let troop = self.game.get_troop(troop_id);
		(troop.do_play)(&mut test_position, TroopInfo { id: troop_id, coord, side }, play);
		let info = test_position.analyze();
	
		// hehehehhaw
		// basically, if any OTHER side is threatening the alpha, then the play was not safe
		!info.threats[alpha].iter().enumerate().filter(|(i, _)| Side::from(*i) != side).map(|(_, b)| *b).reduce(|a, b| a || b).unwrap()
	}

	pub fn fen_string_2p(&self) -> String {
		let mut string = String::new();

		let mut empty = 0;
		for y in 0..self.game.height {
			let y = self.game.height - y - 1;

			for x in 0..self.game.width {
				let coord = Coord::from_xy(&self.game, x, y);
				if let Some((side, troop_id)) = self.tiles[coord] {
					if empty > 0 {
						string.push_str(&empty.to_string());
						empty = 0;
					}

					let troop = self.game.get_troop(troop_id);
					if side == Side::from(0) {
						string.push(troop.char);
					} else {
						string.push(troop.char.to_ascii_lowercase());
					}
				} else {
					empty += 1;
				}
			}

			if empty > 0 {
				string.push_str(&empty.to_string());
				empty = 0;
			}

			if y > 0 {
				string.push('/');
			}
		}

		string
	}
}