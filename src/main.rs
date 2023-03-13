mod chess;
mod classical;
mod gui;

use chess::*;
use classical::*;

fn main() {
	gui::run_gui();

	let king = Troop {
		char: 'K',
		get_plays: get_king_plays,
		do_play: do_king_play,
	};
}
