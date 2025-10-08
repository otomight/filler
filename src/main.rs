mod logger;
mod parser;
mod global;
mod algo;

use std::{io::{self, BufRead, Write}, sync::Mutex};

use crate::{algo::find_placement, global::{ERROR_LOGGER, LOGGER}, logger::Logger, parser::{read_input, read_player}};

fn main() {
	init_logger("stdout.log", "stderr.log");
	let stdin = io::stdin();
	let mut stdout = io::stdout();
	let mut lines_iter = stdin.lock().lines().map(|l| l.unwrap());

	let player = read_player(&mut lines_iter).expect("failed to read player");
	// log(&format!("{} {}", player.mark, player.ghost_mark));
	while let Some(turn) = read_input(&player, &mut lines_iter) {
		// log(&format!("{}", player.mark));
		// log(&format!("{:?}", turn.piece_size));
		// log(&format!("{:?}", turn.piece));
		// log(&format!("{:?}", turn.anfield_size));
		// log(&format!("{:?}", turn.anfield));
		if let Some(p) = find_placement(&turn, player.mark) {
			write!(stdout, "{} {}\n", p.x, p.y).unwrap();
		} else {
			write!(stdout, "0 0\n").unwrap();
		}
		stdout.flush().unwrap();
	}
}

pub fn init_logger(log_path: &str, error_path: &str) {
	if let Err(_) = LOGGER.set(Mutex::new(Logger::new(log_path))) {
		panic!("Logger has already been initialized!");
	}
	if let Err(_) = ERROR_LOGGER.set(Mutex::new(Logger::new(error_path))) {
		panic!("Error logger has already been initialized!");
	}

	// Redirect panic messages and eprintln! to the error logger
	std::panic::set_hook(Box::new(|info| {
		if let Some(mutex) = ERROR_LOGGER.get() {
			let mut err_log = mutex.lock().unwrap();
			err_log.log(&format!("PANIC: {}", info));
		} else {
			eprintln!("PANIC (no logger): {}", info);
		}
	}));
}

pub fn log(msg: &str) {
	if let Some(logger_mutex) = LOGGER.get() {
		let mut logger = logger_mutex.lock().unwrap();
		logger.log(msg);
	}
}

pub fn errlog(msg: &str) {
	if let Some(logger_mutex) = ERROR_LOGGER.get() {
		let mut logger = logger_mutex.lock().unwrap();
		logger.log(msg);
	}
}
