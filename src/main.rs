mod chip8;

fn main() {
	let filename = std::env::args().nth(1).unwrap();
	let mut chip8 = chip8::Chip8::new();
	chip8.run(filename);
}
