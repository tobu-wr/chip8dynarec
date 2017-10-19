extern crate rand;

use chip8::Chip8;

pub struct Interpreter;

impl Interpreter {
	pub fn execute_next_instruction(chip8: &mut Chip8) {
		// fetch
		let high_byte = chip8.memory[chip8.register_pc as usize];
		let low_byte = chip8.memory[chip8.register_pc as usize + 1];
		let opcode = (high_byte >> 4, high_byte & 0x0F, low_byte >> 4, low_byte & 0x0F);
		chip8.register_pc += 2;

		// decode & execute
		let nnn = ((high_byte as u16 & 0x0F) << 8) | low_byte as u16;
		let x = high_byte as usize & 0x0F;
		let y = low_byte as usize >> 4;

		match opcode {
			(0x0, 0x0, 0xE, 0x0) => chip8.display.clear(),
			(0x0, 0x0, 0xE, 0xE) => {
				chip8.register_pc = chip8.stack[chip8.register_sp as usize];
				chip8.register_sp -= 1;
			},
			(0x1, ..) => chip8.register_pc = nnn,
			(0x2, ..) => {
				chip8.register_sp += 1;
				chip8.stack[chip8.register_sp as usize] = chip8.register_pc;
				chip8.register_pc = nnn;
			},
			(0x3, ..) => {
				if chip8.register_v[x] == low_byte {
					chip8.register_pc += 2;
				}
			},
			(0x4, ..) => {
				if chip8.register_v[x] != low_byte {
					chip8.register_pc += 2;
				}
			},
			(0x5, _, _, 0x0) => {
				if chip8.register_v[x] == chip8.register_v[y] {
					chip8.register_pc += 2;
				}
			},
			(0x6, ..) => chip8.register_v[x] = low_byte,
			(0x7, ..) => chip8.register_v[x] = chip8.register_v[x].wrapping_add(low_byte),
			(0x8, _, _, 0x0) => chip8.register_v[x] = chip8.register_v[y],
			(0x8, _, _, 0x1) => chip8.register_v[x] |= chip8.register_v[y],
			(0x8, _, _, 0x2) => chip8.register_v[x] &= chip8.register_v[y],
			(0x8, _, _, 0x3) => chip8.register_v[x] ^= chip8.register_v[y],
			(0x8, _, _, 0x4) => {
				let result = chip8.register_v[x] as u16 + chip8.register_v[y] as u16;
				chip8.register_v[x] = result as u8;
				chip8.register_v[0xF] = (result > 0xFF) as u8;
			},
			(0x8, _, _, 0x5) => {
				chip8.register_v[0xF] = (chip8.register_v[x] >= chip8.register_v[y]) as u8;
				chip8.register_v[x] = chip8.register_v[x].wrapping_sub(chip8.register_v[y]);
			},
			(0x8, _, _, 0x6) => {
				chip8.register_v[0xF] = chip8.register_v[y] & 1;
				chip8.register_v[x] = chip8.register_v[y] >> 1;
			},
			(0x8, _, _, 0x7) => {
				chip8.register_v[0xF] = (chip8.register_v[x] <= chip8.register_v[y]) as u8;
				chip8.register_v[x] = chip8.register_v[y].wrapping_sub(chip8.register_v[x]);
			},
			(0x8, _, _, 0xE) => {
				chip8.register_v[0xF] = chip8.register_v[y] >> 7;
				chip8.register_v[x] = chip8.register_v[y] << 1;
			},
			(0x9, _, _, 0x0) => {
				if chip8.register_v[x] != chip8.register_v[y] {
					chip8.register_pc += 2;
				}
			},
			(0xA, ..) => chip8.register_i = nnn,
			(0xB, ..) => chip8.register_pc = nnn + chip8.register_v[0] as u16,
			(0xC, ..) => chip8.register_v[x] = rand::random::<u8>() & low_byte,
			(0xD, _, _, n) => {
				let sprite = &chip8.memory[chip8.register_i as usize .. chip8.register_i as usize + n as usize];
				chip8.register_v[0xF] = chip8.display.draw_sprite(chip8.register_v[x], chip8.register_v[y], sprite) as u8;
			},
			(0xE, _, 0x9, 0xE) => {
				if chip8.keyboard.is_pressed(chip8.register_v[x]) {
					chip8.register_pc += 2;
				}
			},
			(0xE, _, 0xA, 0x1) => {
				if !chip8.keyboard.is_pressed(chip8.register_v[x]) {
					chip8.register_pc += 2;
				}
			},
			(0xF, _, 0x0, 0x7) => chip8.register_v[x] = chip8.register_dt,
			(0xF, _, 0x0, 0xA) => chip8.register_v[x] = chip8.keyboard.wait_key_press(),
			(0xF, _, 0x1, 0x5) => chip8.register_dt = chip8.register_v[x],
			(0xF, _, 0x1, 0x8) => chip8.register_st = chip8.register_v[x],
			(0xF, _, 0x1, 0xE) => chip8.register_i += chip8.register_v[x] as u16,
			(0xF, _, 0x2, 0x9) => chip8.register_i = chip8.register_v[x] as u16 * 5,
			(0xF, _, 0x3, 0x3) => {
				chip8.memory[chip8.register_i as usize] = chip8.register_v[x] / 100;
				chip8.memory[chip8.register_i as usize + 1] = (chip8.register_v[x] % 100) / 10;
				chip8.memory[chip8.register_i as usize + 2] = chip8.register_v[x] % 10;
			},
			(0xF, _, 0x5, 0x5) => {
				for i in 0..(x + 1) {
					chip8.memory[chip8.register_i as usize] = chip8.register_v[i];
					chip8.register_i += 1;
				}
			},
			(0xF, _, 0x6, 0x5) => {
				for i in 0..(x + 1) {
					chip8.register_v[i] = chip8.memory[chip8.register_i as usize];
					chip8.register_i += 1;
				}
			},
			_ => panic!("unknown opcode")
		}
	}
}
