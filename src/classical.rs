use std::{rc::Rc, sync::Arc};
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
			char: 'Q',
			get_plays: get_queen_plays,
			do_play: do_normal_play,
		}, Troop {
			char: 'R',
			get_plays: get_rook_plays,
			do_play: do_normal_play,
		}, Troop {
			char: 'B',
			get_plays: get_bishop_plays,
			do_play: do_normal_play,
		}, Troop {
			char: 'N',
			get_plays: get_knight_plays,
			do_play: do_normal_play,
		}, Troop {
			char: 'P',
			get_plays: get_pawn_plays,
			do_play: do_pawn_play,
		}],
		multi_castle: false,
	})
}

pub fn setup_classical() -> Position {
	let mut position = Position {
		game: classical_game().clone(),
		tiles: [None; 256],
		alphas: [None; 8],
		recent_plys: [(); 24].map(|_| Ply::None),
		passant: Vec::new(),
		castlings: [(); 8].map(|_| Vec::new()),
	};

	for x in 0..8 {
		position.tiles[Coord::from_xy(&position.game, x, 1)] = Some((Side::from(0), TroopId::from(5)));
		position.tiles[Coord::from_xy(&position.game, x, 6)] = Some((Side::from(1), TroopId::from(5)));
	}

	position.tiles[Coord::from_xy(&position.game, 0, 0)] = Some((Side::from(0), TroopId::from(2)));
	position.tiles[Coord::from_xy(&position.game, 7, 0)] = Some((Side::from(0), TroopId::from(2)));

	position.tiles[Coord::from_xy(&position.game, 1, 0)] = Some((Side::from(0), TroopId::from(4)));
	position.tiles[Coord::from_xy(&position.game, 6, 0)] = Some((Side::from(0), TroopId::from(4)));

	position.tiles[Coord::from_xy(&position.game, 2, 0)] = Some((Side::from(0), TroopId::from(3)));
	position.tiles[Coord::from_xy(&position.game,5, 0)] = Some((Side::from(0), TroopId::from(3)));

	position.tiles[Coord::from_xy(&position.game, 3, 0)] = Some((Side::from(0), TroopId::from(1)));
	position.tiles[Coord::from_xy(&position.game, 4, 0)] = Some((Side::from(0), TroopId::from(0)));


	position.tiles[Coord::from_xy(&position.game, 0, 7)] = Some((Side::from(1), TroopId::from(2)));
	position.tiles[Coord::from_xy(&position.game, 7, 7)] = Some((Side::from(1), TroopId::from(2)));

	position.tiles[Coord::from_xy(&position.game, 1, 7)] = Some((Side::from(1), TroopId::from(4)));
	position.tiles[Coord::from_xy(&position.game, 6, 7)] = Some((Side::from(1), TroopId::from(4)));

	position.tiles[Coord::from_xy(&position.game, 2, 7)] = Some((Side::from(1), TroopId::from(3)));
	position.tiles[Coord::from_xy(&position.game,5, 7)] = Some((Side::from(1), TroopId::from(3)));

	position.tiles[Coord::from_xy(&position.game, 3, 7)] = Some((Side::from(1), TroopId::from(1)));
	position.tiles[Coord::from_xy(&position.game,4,7)] = Some((Side::from(1), TroopId::from(0)));

	position
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

	position.passant = Vec::new();

	ply
}

fn get_king_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let (x, y) = info.coord.decomp(&position.game);

	let mut plays = Vec::new();
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
				plays.push(TroopPlay { to: ncoord, threats: Vec::new() });
				continue;
			};

			if side != info.side {
				plays.push(TroopPlay { to: ncoord, threats: vec![ncoord] });
			}
		}
	}

	plays
}

fn get_knight_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let game = &position.game;
	let (x, y) = info.coord.decomp(&game);

	let mut plays = Vec::new();
	
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
			plays.push(TroopPlay { to: coord, threats: Vec::new() });
			continue;
		};
		
		if side != info.side {
			plays.push(TroopPlay { to: coord, threats: vec![coord] });
		}
	}

	plays
}

fn get_pawn_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let game = &position.game;
	let (x, y) = info.coord.decomp(&game);
	let mut plays = Vec::new();

	let (dy, dash_rank) = match info.side.idx {
		0 => (1, 1),
		1 => (-1, 6),
		_ => panic!("this is not classical chess, bozo"),
	};

	if position.tiles[Coord::from_xy(&game, x, (y as i32 + dy) as u8)].is_none() {
		plays.push(TroopPlay { to: Coord::from_xy(&game, x, (y as i32 + dy) as u8), threats: Vec::new() });

		if y == dash_rank && position.tiles[Coord::from_xy(&game, x, (y as i32 + dy + dy) as u8)].is_none() {
			plays.push(TroopPlay { to: Coord::from_xy(&game, x, (y as i32 + dy + dy) as u8), threats: Vec::new() });
		}
	}

	if x > 1 {
		if let Some((side, _)) = position.tiles[Coord::from_xy(&game, x - 1, (y as i32 + dy) as u8)] {
			if side != info.side {
				plays.push(TroopPlay { to: Coord::from_xy(&game, x - 1, (y as i32 + dy) as u8), threats: vec![Coord::from_xy(&game, x - 1, (y as i32 + dy) as u8)] });
			}
		}

		if position.passant.len() == 3 && x - 1 == position.passant[1].decomp(&game).0 {
			if let Some((side, _)) = position.tiles[position.passant[2]] {
				if side != info.side {
					plays.push(TroopPlay { to: Coord::from_xy(&game, x - 1, (y as i32 + dy) as u8), threats: vec![position.passant[2]] });
				}
			}
		}
	}

	if x < game.width - 1 {
		if let Some((side, _)) = position.tiles[Coord::from_xy(&game, x + 1, (y as i32 + dy) as u8)] {
			if side != info.side {
				plays.push(TroopPlay { to: Coord::from_xy(&game, x + 1, (y as i32 + dy) as u8), threats: vec![Coord::from_xy(&game, x + 1, (y as i32 + dy) as u8)] });
			}
		}

		if position.passant.len() == 3 && x + 1 == position.passant[1].decomp(&game).0 {
			if let Some((side, _)) = position.tiles[position.passant[2]] {
				if side != info.side {
					plays.push(TroopPlay { to: Coord::from_xy(&game, x + 1, (y as i32 + dy) as u8), threats: vec![position.passant[2]] });
				}
			}
		}
	}

	plays
}


fn do_pawn_play(position: &mut Position, info: TroopInfo, play: &TroopPlay) -> MovePly {
	let game = &position.game;
	let ply = MovePly {
		troop: info.id,
		from: info.coord,
		to: play.to,
		kills: play.threats.clone(),
	};

	position.tiles[info.coord] = None;
	position.tiles[play.to] = Some((info.side, info.id));

	for &threat in &play.threats {
		position.tiles[threat] = None;
	}

	if i32::abs(info.coord.decomp(&game).1 as i32 - play.to.decomp(&game).1 as i32) > 1 {
		position.passant = vec![info.coord, play.to, play.to];
	} else {
		position.passant = Vec::new();
	}

	let prom_rank = match info.side.idx {
		0 => 7,
		1 => 0,
		_ => panic!("this is not classical chess, bozo"),
	};

	if play.to.decomp(&game).1 == prom_rank {
		position.tiles[play.to] = Some((info.side, TroopId::from(0)));
	}

	ply
}

fn add_slide_plays(position: &Position, plays: &mut Vec<TroopPlay>, from: Coord, dx: i32, dy: i32) {
	let game = &position.game;
	let (mut x, mut y) = from.decomp(&game);
	let (side, _) = position.tiles[from].unwrap();

	loop {
		x = (x as i32 + dx) as u8;
		y = (y as i32 + dy) as u8;

		// if x < 0; x == u8::MAX >= game.width
		if x >= game.width || y >= game.height {
			break;
		}

		let coord = Coord::from_xy(&game, x, y);

		if let Some((other_side, _)) = position.tiles[coord] {
			if side != other_side {
				plays.push(TroopPlay { to: coord, threats: vec![coord] });
			}

			break;
		}

		plays.push(TroopPlay { to: coord, threats: Vec::new() });
	}
}

fn get_rook_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let mut plays = Vec::new();

	add_slide_plays(position, &mut plays, info.coord, 1, 0);
	add_slide_plays(position, &mut plays, info.coord, -1, 0);
	add_slide_plays(position, &mut plays, info.coord, 0, 1);
	add_slide_plays(position, &mut plays, info.coord, 0, -1);

	plays
}

fn get_bishop_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let mut plays = Vec::new();

	add_slide_plays(position, &mut plays, info.coord, 1, 1);
	add_slide_plays(position, &mut plays, info.coord, -1, 1);
	add_slide_plays(position, &mut plays, info.coord, 1, -1);
	add_slide_plays(position, &mut plays, info.coord, -1, -1);

	plays
}

fn get_queen_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
	let mut plays = Vec::new();

	add_slide_plays(position, &mut plays, info.coord, 1, 0);
	add_slide_plays(position, &mut plays, info.coord, -1, 0);
	add_slide_plays(position, &mut plays, info.coord, 0, 1);
	add_slide_plays(position, &mut plays, info.coord, 0, -1);

	add_slide_plays(position, &mut plays, info.coord, 1, 1);
	add_slide_plays(position, &mut plays, info.coord, -1, 1);
	add_slide_plays(position, &mut plays, info.coord, 1, -1);
	add_slide_plays(position, &mut plays, info.coord, -1, -1);

	plays
}