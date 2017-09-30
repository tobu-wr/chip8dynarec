mod chip8;

use std::env;

fn main() {
	let filename = env::args().nth(1).unwrap();
	let mut chip8 = chip8::Chip8::default();
	chip8.load_rom(filename);
	chip8.run();
}
