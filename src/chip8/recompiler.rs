use std::collections::HashMap;

use chip8::Chip8;
use chip8::codeblock::CodeBlock;
use chip8::codeemitter::CodeEmitter;

pub struct Recompiler {
	code_cache: HashMap<u16, CodeBlock>
}

impl Recompiler {
	pub fn new() -> Recompiler {
		Recompiler {
			code_cache: HashMap::new()
		}
	}

	pub fn execute_next_code_block(&mut self, chip8: &mut Chip8) {
		if !self.code_cache.contains_key(&chip8.register_pc) {
			let code_block = Recompiler::recompile_next_code_block(chip8);
			self.code_cache.insert(chip8.register_pc, code_block);
		}
		self.code_cache[&chip8.register_pc].execute();
	}

	fn recompile_next_code_block(chip8: &Chip8) -> CodeBlock {
		let mut code_emitter = CodeEmitter::new();
		let mut register_pc = chip8.register_pc;

		loop {
			let high_byte = chip8.memory[register_pc as usize];
			let low_byte = chip8.memory[register_pc as usize + 1];
			let opcode = (high_byte >> 4, high_byte & 0x0F, low_byte >> 4, low_byte & 0x0F);
			let nnn = ((high_byte as u16 & 0x0F) << 8) | low_byte as u16;
			let x = high_byte as usize & 0x0F;
			//let y = low_byte as usize >> 4;

			register_pc += 2;

			match opcode {
				(0x0, 0x0, 0xE, 0x0) => {
					unimplemented!();
				},
				(0x0, 0x0, 0xE, 0xE) => {
					unimplemented!();
				},
				(0x1, ..) => {
					unimplemented!();
				},
				(0x2, ..) => {
					register_pc = nnn;
				},
				(0x3, ..) => {
					code_emitter.cmp_imm_with_m8(low_byte, &chip8.register_v[x] as *const u8 as u32);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16 as u32);
					code_emitter.jne(9);
					code_emitter.add_imm_to_m16(2, &chip8.register_pc as *const u16 as u32);
					break;
				},
				(0x4, ..) => {
					unimplemented!();
				},
				(0x5, _, _, 0x0) => {
					unimplemented!();
				},
				(0x6, ..) => code_emitter.mov_imm_to_m8(low_byte, &chip8.register_v[x] as *const u8 as u32),
				(0x7, ..) => code_emitter.add_imm_to_m8(low_byte, &chip8.register_v[x] as *const u8 as u32),
				(0x8, _, _, 0x0) => {
					unimplemented!();
				},
				(0x8, _, _, 0x1) => {
					unimplemented!();
				},
				(0x8, _, _, 0x2) => {
					unimplemented!();
				},
				(0x8, _, _, 0x3) => {
					unimplemented!();
				},
				(0x8, _, _, 0x4) => {
					unimplemented!();
				},
				(0x8, _, _, 0x5) => {
					unimplemented!();
				},
				(0x8, _, _, 0x6) => {
					unimplemented!();
				},
				(0x8, _, _, 0x7) => {
					unimplemented!();
				},
				(0x8, _, _, 0xE) => {
					unimplemented!();
				},
				(0x9, _, _, 0x0) => {
					unimplemented!();
				},
				(0xA, ..) => {
					unimplemented!();
				},
				(0xB, ..) => {
					unimplemented!();
				},
				(0xC, ..) => {
					unimplemented!();
				},
				(0xD, _, _, n) => {
					unimplemented!();
				},
				(0xE, _, 0x9, 0xE) => {
					unimplemented!();
				},
				(0xE, _, 0xA, 0x1) => {
					unimplemented!();
				},
				(0xF, _, 0x0, 0x7) => {
					unimplemented!();
				},
				(0xF, _, 0x0, 0xA) => {
					unimplemented!();
				},
				(0xF, _, 0x1, 0x5) => {
					unimplemented!();
				},
				(0xF, _, 0x1, 0x8) => {
					unimplemented!();
				},
				(0xF, _, 0x1, 0xE) => {
					unimplemented!();
				},
				(0xF, _, 0x2, 0x9) => {
					code_emitter.mov_imm_to_al(5);
					code_emitter.mul_al_with_m(&chip8.register_v[x] as *const u8 as u32);
					code_emitter.mov_ax_to_m(&chip8.register_i as *const u16 as u32);
				},
				(0xF, _, 0x3, 0x3) => {
					unimplemented!();
				},
				(0xF, _, 0x5, 0x5) => {
					unimplemented!();
				},
				(0xF, _, 0x6, 0x5) => {
					unimplemented!();
				},
				_ => panic!("unknown opcode")
			}
		}

		code_emitter.ret();

		CodeBlock::new(code_emitter.raw_code)
	}
}
