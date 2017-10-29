use std::collections::HashMap;

use chip8::Chip8;
use chip8::codeblock::CodeBlock;
use chip8::codeemitter::CodeEmitter;

pub struct Recompiler {
	code_cache: HashMap<u16, CodeBlock>,
	draw_sprite_interrupt: bool,
	clear_interrupt: bool
}

impl Recompiler {
	pub fn new() -> Recompiler {
		Recompiler {
			code_cache: HashMap::new(),
			draw_sprite_interrupt: false,
			clear_interrupt: false
		}
	}

	pub fn execute_next_code_block(&mut self, chip8: &mut Chip8) {
		if !self.code_cache.contains_key(&chip8.register_pc) {
			let code_block = self.recompile_next_code_block(chip8);
			self.code_cache.insert(chip8.register_pc, code_block);
		}
		self.code_cache[&chip8.register_pc].execute();

		if self.draw_sprite_interrupt {
			let high_byte = chip8.memory[chip8.register_pc as usize - 2] as usize;
			let low_byte = chip8.memory[chip8.register_pc as usize - 1] as usize;
			let x = high_byte & 0x0F;
			let y = low_byte >> 4;
			let n = low_byte & 0x0F;
			let sprite = &chip8.memory[chip8.register_i as usize .. chip8.register_i as usize + n];
			chip8.register_v[0xF] = chip8.display.draw_sprite(chip8.register_v[x], chip8.register_v[y], sprite) as u8;
			self.draw_sprite_interrupt = false;
		} 
		else if self.clear_interrupt {
			chip8.display.clear();
			self.clear_interrupt = false;
		}
	}

	fn recompile_next_code_block(&mut self, chip8: &Chip8) -> CodeBlock {
		let mut code_emitter = CodeEmitter::new();
		let mut register_pc = chip8.register_pc;

		code_emitter.pusha();

		loop {
			let high_byte = chip8.memory[register_pc as usize];
			let low_byte = chip8.memory[register_pc as usize + 1];
			let opcode = (high_byte >> 4, high_byte & 0x0F, low_byte >> 4, low_byte & 0x0F);
			let nnn = ((high_byte as u16 & 0x0F) << 8) | low_byte as u16;
			let x = high_byte as usize & 0x0F;
			let y = low_byte as usize >> 4;

			register_pc += 2;

			match opcode {
				(0x0, 0x0, 0xE, 0x0) => {
					code_emitter.mov_imm_to_m8(1, &self.clear_interrupt as *const bool as *const u8);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16);
					break;
				},
				(0x0, 0x0, 0xE, 0xE) => {
					code_emitter.movzx_m8_to_esi(&chip8.register_sp as *const i8 as *const u8);
					code_emitter.mov_imm_to_edi(&chip8.stack as *const [u16; 16] as u32);
					code_emitter.mov_m_to_ax_edi2esi();
					code_emitter.mov_ax_to_m(&chip8.register_pc as *const u16);
					code_emitter.sub_imm_to_m8(1, &chip8.register_sp as *const i8 as *const u8);
					break;
				},
				(0x1, ..) => register_pc = nnn,
				(0x2, ..) => {
					code_emitter.add_imm_to_m8(1, &chip8.register_sp as *const i8 as *const u8);
					code_emitter.movzx_m8_to_esi(&chip8.register_sp as *const i8 as *const u8);
					code_emitter.mov_imm_to_edi(&chip8.stack as *const [u16; 16] as u32);
					code_emitter.mov_imm_to_m16_edi2esi(register_pc);
					register_pc = nnn;
				},
				(0x3, ..) => {
					code_emitter.cmp_imm_with_m8(low_byte, &chip8.register_v[x] as *const u8);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16);
					code_emitter.jne(9);
					code_emitter.add_imm_to_m16(2, &chip8.register_pc as *const u16);
					break;
				},
				(0x4, ..) => {
					code_emitter.cmp_imm_with_m8(low_byte, &chip8.register_v[x] as *const u8);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16);
					code_emitter.je(9);
					code_emitter.add_imm_to_m16(2, &chip8.register_pc as *const u16);
					break;
				},
				(0x5, _, _, 0x0) => {
					unimplemented!();
				},
				(0x6, ..) => code_emitter.mov_imm_to_m8(low_byte, &chip8.register_v[x] as *const u8),
				(0x7, ..) => code_emitter.add_imm_to_m8(low_byte, &chip8.register_v[x] as *const u8),
				(0x8, _, _, 0x0) => {
					code_emitter.mov_m_to_al(&chip8.register_v[y] as *const u8);
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
				},
				(0x8, _, _, 0x1) => {
					unimplemented!();
				},
				(0x8, _, _, 0x2) => {
					code_emitter.mov_m_to_al(&chip8.register_v[y] as *const u8);
					code_emitter.and_m_al(&chip8.register_v[x] as *const u8);
				},
				(0x8, _, _, 0x3) => {
					unimplemented!();
				},
				(0x8, _, _, 0x4) => {
					code_emitter.movzx_m_to_ax(&chip8.register_v[x] as *const u8);
					code_emitter.movzx_m_to_cx(&chip8.register_v[y] as *const u8);
					code_emitter.add_cx_to_ax();
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
					code_emitter.cmp_ax_with_imm(0xFF);
					code_emitter.seta_m(&chip8.register_v[0xF] as *const u8);
				},
				(0x8, _, _, 0x5) => {
					code_emitter.mov_m_to_al(&chip8.register_v[x] as *const u8);
					code_emitter.sub_m_to_al(&chip8.register_v[y] as *const u8);
					code_emitter.setae_m(&chip8.register_v[0xF] as *const u8);
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
				},
				(0x8, _, _, 0x6) => {
					unimplemented!();
				},
				(0x8, _, _, 0x7) => {
					code_emitter.mov_m_to_al(&chip8.register_v[y] as *const u8);
					code_emitter.sub_m_to_al(&chip8.register_v[x] as *const u8);
					code_emitter.setae_m(&chip8.register_v[0xF] as *const u8);
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
				},
				(0x8, _, _, 0xE) => {
					code_emitter.mov_m_to_al(&chip8.register_v[y] as *const u8);
					code_emitter.shr_al_imm(7);
					code_emitter.mov_al_to_m(&chip8.register_v[0xF] as *const u8);
					code_emitter.add_al_to_al();
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
				},
				(0x9, _, _, 0x0) => {
					code_emitter.mov_m_to_al(&chip8.register_v[x] as *const u8);
					code_emitter.cmp_m_with_al(&chip8.register_v[y] as *const u8);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16);
					code_emitter.je(9);
					code_emitter.add_imm_to_m16(2, &chip8.register_pc as *const u16);
					break;
				},
				(0xA, ..) => code_emitter.mov_imm_to_m16(nnn, &chip8.register_i as *const u16),
				(0xB, ..) => {
					unimplemented!();
				},
				(0xC, ..) => {
					code_emitter.rdrand_ax();
					code_emitter.and_al_imm(low_byte);
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8)
				},
				(0xD, ..) => {
					code_emitter.mov_imm_to_m8(1, &self.draw_sprite_interrupt as *const bool as *const u8);
					code_emitter.mov_imm_to_m16(register_pc, &chip8.register_pc as *const u16);
					break;
				},
				(0xE, _, 0x9, 0xE) => {
					unimplemented!();
				},
				(0xE, _, 0xA, 0x1) => {
					unimplemented!();
				},
				(0xF, _, 0x0, 0x7) => {
					code_emitter.mov_m_to_al(&chip8.register_dt as *const u8);
					code_emitter.mov_al_to_m(&chip8.register_v[x] as *const u8);
				},
				(0xF, _, 0x0, 0xA) => {
					unimplemented!();
				},
				(0xF, _, 0x1, 0x5) => {
					code_emitter.mov_m_to_al(&chip8.register_v[x] as *const u8);
					code_emitter.mov_al_to_m(&chip8.register_dt as *const u8);
				},
				(0xF, _, 0x1, 0x8) => {
					unimplemented!();
				},
				(0xF, _, 0x1, 0xE) => {
					code_emitter.movzx_m_to_ax(&chip8.register_v[x] as *const u8);
					code_emitter.add_ax_to_m(&chip8.register_i as *const u16);
				},
				(0xF, _, 0x2, 0x9) => {
					code_emitter.mov_imm_to_al(5);
					code_emitter.mul_m8(&chip8.register_v[x] as *const u8);
					code_emitter.mov_ax_to_m(&chip8.register_i as *const u16);
				},
				(0xF, _, 0x3, 0x3) => {
					unimplemented!();
				},
				(0xF, _, 0x5, 0x5) => {
					code_emitter.xor_cl_cl();
					code_emitter.movzx_cl_to_esi();
					code_emitter.mov_imm_to_edi(&chip8.register_v as *const [u8; 16] as u32);
					code_emitter.mov_m_to_al_ediesi();
					code_emitter.movzx_m16_to_esi(&chip8.register_i as *const u16);
					code_emitter.mov_imm_to_edi(&chip8.memory as *const [u8; 0x1000] as u32);
					code_emitter.mov_al_to_m_ediesi();
					code_emitter.add_imm_to_m16(1, &chip8.register_i as *const u16);
					code_emitter.add_imm_to_cl(1);
					code_emitter.cmp_cl_with_imm(x as u8 + 1);
					code_emitter.jne(-30);
				},
				(0xF, _, 0x6, 0x5) => {
					code_emitter.xor_cl_cl();
					code_emitter.movzx_m16_to_esi(&chip8.register_i as *const u16);
					code_emitter.mov_imm_to_edi(&chip8.memory as *const [u8; 0x1000] as u32);
					code_emitter.mov_m_to_al_ediesi();
					code_emitter.movzx_cl_to_esi();
					code_emitter.mov_imm_to_edi(&chip8.register_v as *const [u8; 16] as u32);
					code_emitter.mov_al_to_m_ediesi();
					code_emitter.add_imm_to_m16(1, &chip8.register_i as *const u16);
					code_emitter.add_imm_to_cl(1);
					code_emitter.cmp_cl_with_imm(x as u8 + 1);
					code_emitter.jne(-30);
				},
				_ => panic!("unknown opcode")
			}
		}

		code_emitter.popa();
		code_emitter.ret();

		CodeBlock::new(code_emitter.raw_code)
	}
}
