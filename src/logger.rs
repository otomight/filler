use std::{fs::{File, OpenOptions}, io::{BufWriter, Write}};

pub struct Logger {
	buf_writer: BufWriter<File>
}

impl Logger {
	pub fn new(filename: &str) -> Logger {
		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(filename)
			.expect("Failed to open log file");
		let buf_writer = BufWriter::new(file);
		return Logger{buf_writer}
	}

	pub fn write(&mut self, s: &str) {
		writeln!(self.buf_writer, "{}", s).expect("Failed to write log");
	}

	pub fn flush(&mut self) {
		self.buf_writer.flush().expect("Failed to flush log");
	}

	pub fn log(&mut self, s: &str) {
		self.write(s);
		self.flush();
	}
}
