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

	position.tiles[34] = Some((Side::from(0), TroopId::from(0)));
	position.tiles[36] = Some((Side::from(1), TroopId::from(0)));
	position.tiles[19] = Some((Side::from(1), TroopId::from(1)));

	println!("{}", position.fen_string_2p());

	let plays = position.troop_plays(Coord::from(19));

	for play in &plays {
		print!("{}", play.to.fmt(&game));
		for threat in &play.threats {
			print!(", x{}", threat.fmt(&game));
		}

		println!();
	}
}
