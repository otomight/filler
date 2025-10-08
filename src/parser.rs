use crate::{global::{GHOST_MARKS, MARKS}};

#[derive(Debug)]
pub struct Size {
	pub w: usize,
	pub h: usize,
}

pub struct Player {
	pub mark: char,
	pub ghost_mark: char,
}

pub struct Turn {
	pub anfield: Vec<Vec<char>>,
	pub anfield_size: Size,
	pub piece: Vec<Vec<char>>,
	pub piece_size: Size,
}

pub fn read_input(
	player: &Player, lines_iter: &mut impl Iterator<Item = String>
) -> Option<Turn> {
	let mut turn = Turn {
		anfield: Vec::new(),
		anfield_size: Size{w: 0, h: 0},
		piece: Vec::new(),
		piece_size: Size{w: 0, h: 0},
	};

	// --- Read until we find "Anfield" ---
	let mut line = lines_iter.next()?;
	while !line.starts_with("Anfield") {
		line = match lines_iter.next() {
			Some(l) => l,
			None => return None,
		};
	}

	// --- Parse field size ---
	let parts: Vec<&str> = line.split_whitespace().collect();
	let field_w: usize = parts[1].parse().unwrap();
	let field_h: usize = parts[2].trim_end_matches(':').parse().unwrap();
	turn.anfield_size = Size{w: field_w, h: field_h};
	// --- Read the field ---
	lines_iter.next(); // skip column indexes
	for _ in 0..field_h {
		if let Some(map_line) = lines_iter.next() {
			let parts: Vec<&str> = map_line.split_whitespace().collect();
			turn.anfield.push(parts[1].to_string().chars().collect());
		} else {
			return None;
		}
	}

	// --- Find the "Piece" line ---
	let mut line = lines_iter.next()?;
	while !line.starts_with("Piece") {
		line = match lines_iter.next() {
			Some(l) => l,
			None => return None,
		};
	}

	// --- Parse piece height ---
	let parts: Vec<&str> = line.split_whitespace().collect();
	let piece_w: usize = parts[1].parse().unwrap();
	let piece_h: usize = parts[2].trim_end_matches(':').parse().unwrap();
	turn.piece_size = Size{w: piece_w, h: piece_h};
	// --- Read the piece ---
	for _ in 0..piece_h {
		if let Some(piece_line) = lines_iter.next() {
			turn.piece.push(piece_line.chars().collect());
		} else {
			return None;
		}
	}

	format_anfield(&player, &mut turn);
	format_piece(&player, &mut turn);

	return Some(turn);
}

pub fn read_player(lines_iter: &mut impl Iterator<Item = String>) -> Option<Player> {
	let mut line = lines_iter.next()?;
	while !line.ends_with("filler]") {
		line = match lines_iter.next() {
			Some(l) => l,
			None => return None,
		};
	}
	let parts: Vec<&str> = line.split_whitespace().collect();
	let player_num: u8 = parts[2].trim_start_matches('p').parse().unwrap();
	let mark = *MARKS.get(&player_num).expect("Failed to get the mark of player");
	let ghost_mark = *GHOST_MARKS.get(&mark).expect("Failed to get the ghost mark of player");
	return Some(Player {mark, ghost_mark});
}

fn format_piece(player: &Player, turn: &mut Turn) {
	for row in turn.piece.iter_mut() {
		for c in row {
			if *c != '.' {
				*c = player.mark;
			}
		}
	}
}

fn format_anfield(player: &Player, turn: &mut Turn) {
	for row in turn.anfield.iter_mut() {
		for c in row {
			if *c == player.ghost_mark {
				*c = player.mark;
			}
		}
	}
}
