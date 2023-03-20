mod chess;
mod classical;

use chess::*;
use classical::*;

fn main() {
	let recent_plys = [(); 24].map(|_| Ply::None);
	let castlings = [(); 8].map(|_| Vec::new());

	let game = classical_game();

	let mut position = Position {
		game: game.clone(),
		tiles: [None; 256],
		alphas: [None; 8],
		recent_plys,
		passant: Vec::new(),
		castlings,
	};

	position.alphas[0] = Some(Coord::from(34));
	position.alphas[1] = Some(Coord::from(36));

	position.tiles[34] = Some((Side::from(0), TroopId::from(0)));
	position.tiles[36] = Some((Side::from(1), TroopId::from(0)));

	position.tiles[19] = Some((Side::from(1), TroopId::from(1)));
	position.tiles[13] = Some((Side::from(0), TroopId::from(2)));
	position.tiles[30] = Some((Side::from(1), TroopId::from(2)));


	let plays = position.troop_plays(Coord::from(13));
	position.troop_play(Coord::from(13), &plays[1]);
	let plays = position.troop_plays(Coord::from(13));
	
	println!("{}", position.fen_string_2p());
	println!("W: {}, B: {}", position.in_check(Side::from(0)), position.in_check(Side::from(1)));

	for play in &plays {
		if !position.is_play_safe(Coord::from(13), play) {
			print!("! ");
		}

		print!("{}", play.to.fmt(&game));
		for threat in &play.threats {
			print!(", x{}", threat.fmt(&game));
		}

		println!();
	}

	println!();

	let plays = position.troop_plays(Coord::from(30));

	for play in &plays {
		if !position.is_play_safe(Coord::from(30), play) {
			print!("! ");
		}

		print!("{}", play.to.fmt(&game));
		for threat in &play.threats {
			print!(", x{}", threat.fmt(&game));
		}

		println!();
	}
}
