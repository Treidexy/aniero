mod chess;
mod classical;

use chess::*;
use classical::*;

fn main() {
	let position = setup_classical();
	println!("{}", position.fen_string_2p());
}
