use std::fs::File;
use std::io::prelude::*;

const MEMORY_SIZE: usize = 0x1000;

pub struct Chip8 {
	memory: [u8; MEMORY_SIZE]
}

impl Chip8 {
	pub fn new() -> Chip8 {
		Chip8 {
			memory: [0; MEMORY_SIZE]
		}
	}

	pub fn load_rom(&mut self, filename: String) {
		// read file
		let mut file = File::open(filename).expect("file not found");
		let mut buffer: Vec<u8> = Vec::new();
		file.read_to_end(&mut buffer).expect("something went wrong reading the file");

		// copy to memory
		let mut i = 0x200;
		for byte in buffer {
			self.memory[i] = byte;
			i += 1;
		}
	}

	pub fn run(&mut self) {
		// TODO
		// ...
	}
}
