use crate::parser::Turn;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub struct Position {
	pub x: usize, // column
	pub y: usize, // row
}

/// Find the best placement following the rules:
/// - prefer candidates built from our border cells (fast)
/// - score by proximity to enemy border (closer = better)
/// - fallback to full scan if no candidate valid
pub fn find_placement(turn: &Turn, mark: char) -> Option<Position> {
	// piece must fit in the map at least somewhere
	let pw = turn.piece_size.w;
	let ph = turn.piece_size.h;
	let mw = turn.anfield_size.w;
	let mh = turn.anfield_size.h;
	if pw == 0 || ph == 0 || pw > mw || ph > mh {
		return None;
	}

	// get border cells for us and enemy
	let (my_borders, enemy_borders) = extract_borders(turn, mark);

	// precompute non-empty piece cells
	let piece_cells = get_piece_cells(turn);
	if piece_cells.is_empty() {
		return None;
	}

	// bounds for reference top-left so piece fully inside
	let max_rx = mw - pw;
	let max_ry = mh - ph;

	// Build candidates by aligning each piece cell to each my_border cell:
	// ref = border - piece_cell
	let mut candidates: HashSet<(usize, usize)> = HashSet::new();
	for &(bx, by) in &my_borders {
		for &(pcx, pcy) in &piece_cells {
			let rx_isize = bx as isize - pcx as isize;
			let ry_isize = by as isize - pcy as isize;
			if rx_isize < 0 || ry_isize < 0 { continue; }
			let rx = rx_isize as usize;
			let ry = ry_isize as usize;
			if rx <= max_rx && ry <= max_ry {
				candidates.insert((rx, ry));
			}
		}
	}

	// Evaluate candidates first (prioritize these)
	let mut best: Option<(usize, usize)> = None;
	let mut best_score = i32::MIN;

	for &(rx, ry) in &candidates {
		if is_valid_placement(turn, mark, rx, ry) {
			let score = score_by_enemy_distance(&enemy_borders, rx, ry, &piece_cells);
			if score > best_score {
				best_score = score;
				best = Some((rx, ry));
			}
		}
	}

	// If none found, fallback to full scan (feasible ref positions)
	if best.is_none() {
		for ry in 0..=max_ry {
			for rx in 0..=max_rx {
				if is_valid_placement(turn, mark, rx, ry) {
					let score = score_by_enemy_distance(&enemy_borders, rx, ry, &piece_cells);
					if score > best_score {
						best_score = score;
						best = Some((rx, ry));
					}
				}
			}
		}
	}

	best.map(|(x, y)| Position { x, y })
}

/// Get piece non-empty cells as (px, py) coords
fn get_piece_cells(turn: &Turn) -> Vec<(usize, usize)> {
	let mut cells = Vec::new();
	for py in 0..turn.piece_size.h {
		for px in 0..turn.piece_size.w {
			if turn.piece[py][px] != '.' {
				cells.push((px, py));
			}
		}
	}
	cells
}

/// Extract both my border cells and enemy border cells
/// Returns (my_borders, enemy_borders) where each is Vec<(x,y)> (col,row)
fn extract_borders(turn: &Turn, my_mark: char) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
	let mut my = Vec::new();
	let mut enemy = Vec::new();

	for y in 0..turn.anfield_size.h {
		for x in 0..turn.anfield_size.w {
			let c = turn.anfield[y][x];
			if c == '.' { continue; } // empty not border
			if has_adjacent_dot(turn, x, y) {
				if c == my_mark {
					my.push((x, y));
				} else {
					// any non-dot and not equal to my_mark is enemy
					enemy.push((x, y));
				}
			}
		}
	}

	(my, enemy)
}

/// True if any 4-neighbor is '.'
fn has_adjacent_dot(turn: &Turn, x: usize, y: usize) -> bool {
	let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
	for (dx, dy) in dirs.iter() {
		let nx = x as i32 + dx;
		let ny = y as i32 + dy;
		if nx >= 0 && ny >= 0 && nx < turn.anfield_size.w as i32 && ny < turn.anfield_size.h as i32 {
			if turn.anfield[ny as usize][nx as usize] == '.' {
				return true;
			}
		}
	}
	false
}

/// validate placement under the rules:
/// - piece placed fully inside map
/// - no overlap with enemy (any char != '.' and != mark)
/// - exactly one overlap with our mark
fn is_valid_placement(turn: &Turn, mark: char, ref_x: usize, ref_y: usize) -> bool {
	let mut overlap_count: usize = 0;

	for py in 0..turn.piece_size.h {
		for px in 0..turn.piece_size.w {
			if turn.piece[py][px] == '.' { continue; }

			let ax = ref_x + px;
			let ay = ref_y + py;

			// bounds check (we ensure ref_x/ref_y are feasible in caller)
			if ax >= turn.anfield_size.w || ay >= turn.anfield_size.h {
				return false;
			}

			let cell = turn.anfield[ay][ax];
			if cell == '.' {
				// ok: placing in empty cell
			} else if cell == mark {
				overlap_count += 1;
			} else {
				// any other char is enemy -> invalid
				return false;
			}
		}
	}

	overlap_count == 1
}

/// Score candidate by minimum Manhattan distance from any placed piece cell to any enemy border cell.
/// Closer => higher score. If no enemy border known, prefer larger coverage.
fn score_by_enemy_distance(enemy_borders: &[(usize, usize)], ref_x: usize, ref_y: usize, piece_cells: &[(usize, usize)]) -> i32 {
	let coverage = piece_cells.len() as i32;
	if enemy_borders.is_empty() {
		// no enemy borders visible: score by coverage only
		return coverage;
	}

	let mut min_dist = i32::MAX;
	for &(pcx, pcy) in piece_cells {
		let ax = ref_x + pcx;
		let ay = ref_y + pcy;
		for &(ex, ey) in enemy_borders {
			let d = manhattan_distance(ax, ay, ex, ey);
			if d < min_dist { min_dist = d; }
		}
	}

	let capped = if min_dist > 100 { 100 } else { min_dist };
	(100 - capped) + coverage * 5
}

#[inline]
fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
	(x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}
