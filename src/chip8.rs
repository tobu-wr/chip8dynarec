use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
pub struct Chip8 {
	rom: Vec<u8>
}

impl Chip8 {
	pub fn load_rom(&mut self, filename: String) {
		let mut file = File::open(filename).expect("file not found");
		file.read_to_end(&mut self.rom).expect("something went wrong reading the file");
	}

	pub fn run(&mut self) {
		// TODO
		// ...
	}
}
