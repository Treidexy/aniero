use crate::{chess::{Game, Position, Side, TroopId, Ply}, classical::{classical_game}};

use eframe::egui;
use egui_extras::RetainedImage;

pub fn run_gui() {
	let options = eframe::NativeOptions::default();

	let image = RetainedImage::from_svg_bytes("assets/wK.svg", include_bytes!("../assets/wK.svg")).unwrap();

	eframe::run_native("Chess", options, Box::new(|_cc| {
		let recent_plys = [(); 24].map(|_| Ply::None);
		let castlings = [(); 8].map(|_| Vec::new());

		let game = classical_game();

		let mut app = Box::new(ChessApp {
			image,
			game,
			position: None,
		});
	
		let mut position = Position {
			game: &app.as_ref().game,
			tiles: [None; 256],
			alphas: [None; 8],
			recent_plys,
			passant: Vec::new(),
			castlings,
		};
	
		position.tiles[34] = Some((Side::from(0), TroopId::from(0)));
		position.tiles[37] = Some((Side::from(1), TroopId::from(0)));

		app.as_mut().position = Some(position);
	
		app
	})).unwrap();
}

struct ChessApp<'a> {
	image: RetainedImage,
	game: Game,
	position: Option<Position<'a>>,
}

impl<'a> eframe::App for ChessApp<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
			ui.image(self.image.texture_id(ctx), [100.0, 100.0]);
		});
    }
}