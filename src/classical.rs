use crate::chess::*;

pub fn classical_game() -> Game {
	Game {
		width: 8,
		height: 8,
		troops: vec![Troop {
			char: 'K',
			get_plays: get_king_plays,
			do_play: do_king_play,
		}],
		multi_castle: false,
	}
}

pub fn get_king_plays(position: &Position, info: TroopInfo) -> Vec<TroopPlay> {
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

pub fn do_king_play(position: &mut Position, info: TroopInfo, play: &TroopPlay) -> MovePly {
	let mut ply = MovePly {
		troop: info.id,
		from: info.coord,
		to: play.to,
		kills: play.threats.clone(),
	};

	if play.threats.len() > 0 {
		let Some((_, _)) = position.tiles[play.threats[0]] else {
			panic!("King play has no threat");
		};
	}

	position.tiles[info.coord] = None;
	position.tiles[play.to] = Some((info.side, info.id));

	ply
}