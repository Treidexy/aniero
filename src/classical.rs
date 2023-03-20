use std::rc::Rc;
use crate::chess::*;

pub fn classical_game() -> Rc<Game> {
	Rc::new(Game {
		width: 8,
		height: 8,
		troops: vec![Troop {
			char: 'K',
			get_plays: get_king_plays,
			do_play: do_normal_play,
		}, Troop {
			char: 'N',
			get_plays: get_knight_plays,
			do_play: do_normal_play,
		}],
		multi_castle: false,
	})
}

fn do_normal_play(position: &mut Position, info: TroopInfo, play: &TroopPlay) -> MovePly {
	let ply = MovePly {
		troop: info.id,
		from: info.coord,
		to: play.to,
		kills: play.threats.clone(),
	};

	position.tiles[info.coord] = None;
	position.tiles[play.to] = Some((info.side, info.id));

	ply
}

fn get_king_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let (x, y) = info.coord.decomp(&position.game);

	let mut moves = Vec::new();
	for dx in -1..2 {
		for dy in -1..2 {
			if dx == 0 && dy == 0 {
				continue;
			}

			let nx = x as i32 + dx;
			let ny = y as i32 + dy;
			if nx < 0 || ny < 0 || nx >= position.game.width as i32 || ny >= position.game.height as i32 {
				continue;
			}
			let ncoord = Coord::from_xy(&position.game, nx as u8, ny as u8);


			let Some((side, _)) = position.tiles[ncoord] else {
				moves.push(TroopPlay { to: ncoord, threats: Vec::new() });
				continue;
			};

			if side != info.side {
				moves.push(TroopPlay { to: ncoord, threats: vec![ncoord] });
			}
		}
	}

	moves
}

fn get_knight_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let game = &position.game;
	let (x, y) = info.coord.decomp(&game);

	let mut moves = Vec::new();
	
	for (dx, dy) in [
		(-2, -1), (-2, 1),
		(2, -1), (2, 1),
		(-1, -2), (-1, 2),
		(1, -2), (1, 2)
		] {
		if x as i32 + dx < 0 || y as i32 + dy < 0 || x as i32 + dx >= game.width as i32 || y as i32 + dy >= game.height as i32 {
			continue;
		}

		let nx = (x as i32 + dx) as u8;
		let ny = (y as i32 + dy) as u8;

		let coord = Coord::from_xy(&game, nx, ny);

		let Some((side, _)) = position.tiles[coord] else {
			moves.push(TroopPlay { to: coord, threats: Vec::new() });
			continue;
		};
		
		if side != info.side {
			moves.push(TroopPlay { to: coord, threats: vec![coord] });
		}
	}

	moves
}

fn get_pawn_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let game = &position.game;
	let (x, y) = info.coord.decomp(&game);
	let mut moves = Vec::new();

	let (dy, dash_rank) = match info.side.idx {
		0 => (1, 1),
		1 => (-1, 7),
		_ => panic!("this is not classical chess, bozo"),
	};

	if position.tiles[Coord::from_xy(&game, x, (y as i32 + dy) as u8)].is_none() {
		moves.push(TroopPlay { to: Coord::from_xy(&game, x, (y as i32 + dy) as u8), threats: Vec::new() });

		if y == dash_rank && position.tiles[Coord::from_xy(&game, x, (y as i32 + dy + dy) as u8)].is_none() {
			moves.push(TroopPlay { to: Coord::from_xy(&game, x, (y as i32 + dy + dy) as u8), threats: Vec::new() });
		}
	}



	moves
}