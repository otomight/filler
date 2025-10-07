use std::{collections::HashMap, sync::{LazyLock, Mutex, OnceLock}};

use crate::logger::Logger;

pub static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();
pub static ERROR_LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub static MARKS: LazyLock<HashMap<u8, char>> = LazyLock::new(|| {
	let mut map = HashMap::new();
	map.insert(1, '@');
	map.insert(2, '$');
	map
});

pub static GHOST_MARKS: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
	let mut map = HashMap::new();
	map.insert('@', 'a');
	map.insert('$', 's');
	map
});
