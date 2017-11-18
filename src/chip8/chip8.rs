extern crate sdl2;

use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

#[cfg(feature="debugger")]
use std::process::Command;

use chip8::MEMORY_SIZE;
use chip8::ROM_START_ADDRESS;
use chip8::keyboard::Keyboard;
use chip8::display::Display;

#[cfg(feature="interpreter")]
use chip8::interpreter::Interpreter;

#[cfg(not(feature="interpreter"))]
use chip8::recompiler::Recompiler;

const STACK_SIZE: usize = 16;
const V_REGISTERS_COUNT: usize = 16;

pub struct Chip8 {
	pub memory: [u8; MEMORY_SIZE],
	pub stack: [u16; STACK_SIZE],
	pub register_v: [u8; V_REGISTERS_COUNT],
	pub register_i: u16,
	pub register_dt: u8,
	pub register_st: u8,
	pub register_pc: u16,
	pub register_sp: u8,
	pub keyboard: Keyboard,
	pub display: Display
}

impl Chip8 {
	pub fn new() -> Chip8 {
		let sdl_context = sdl2::init().unwrap();
		
		let mut chip8 = Chip8 {
			memory: [0; MEMORY_SIZE],
			stack: [0; STACK_SIZE],
			register_v: [0; V_REGISTERS_COUNT],
			register_i: 0,
			register_dt: 0,
			register_st: 0,
			register_pc: ROM_START_ADDRESS,
			register_sp: 0xFF,
			keyboard: Keyboard::new(&sdl_context),
			display: Display::new(&sdl_context)
		};

		chip8.memory[0x00] = 0xF0;
		chip8.memory[0x01] = 0x90;
		chip8.memory[0x02] = 0x90;
		chip8.memory[0x03] = 0x90;
		chip8.memory[0x04] = 0xF0;

		chip8.memory[0x05] = 0x20;
		chip8.memory[0x06] = 0x60;
		chip8.memory[0x07] = 0x20;
		chip8.memory[0x08] = 0x20;
		chip8.memory[0x09] = 0x70;

		chip8.memory[0x0A] = 0xF0;
		chip8.memory[0x0B] = 0x10;
		chip8.memory[0x0C] = 0xF0;
		chip8.memory[0x0D] = 0x80;
		chip8.memory[0x0E] = 0xF0;

		chip8.memory[0x0F] = 0xF0;
		chip8.memory[0x10] = 0x10;
		chip8.memory[0x11] = 0xF0;
		chip8.memory[0x12] = 0x10;
		chip8.memory[0x13] = 0xF0;

		chip8.memory[0x14] = 0x90;
		chip8.memory[0x15] = 0x90;
		chip8.memory[0x16] = 0xF0;
		chip8.memory[0x17] = 0x10;
		chip8.memory[0x18] = 0x10;

		chip8.memory[0x19] = 0xF0;
		chip8.memory[0x1A] = 0x80;
		chip8.memory[0x1B] = 0xF0;
		chip8.memory[0x1C] = 0x10;
		chip8.memory[0x1D] = 0xF0;

		chip8.memory[0x1E] = 0xF0;
		chip8.memory[0x1F] = 0x80;
		chip8.memory[0x20] = 0xF0;
		chip8.memory[0x21] = 0x90;
		chip8.memory[0x22] = 0xF0;

		chip8.memory[0x23] = 0xF0;
		chip8.memory[0x24] = 0x10;
		chip8.memory[0x25] = 0x20;
		chip8.memory[0x26] = 0x40;
		chip8.memory[0x27] = 0x40;

		chip8.memory[0x28] = 0xF0;
		chip8.memory[0x29] = 0x90;
		chip8.memory[0x2A] = 0xF0;
		chip8.memory[0x2B] = 0x90;
		chip8.memory[0x2C] = 0xF0;

		chip8.memory[0x2D] = 0xF0;
		chip8.memory[0x2E] = 0x90;
		chip8.memory[0x2F] = 0xF0;
		chip8.memory[0x30] = 0x10;
		chip8.memory[0x31] = 0xF0;

		chip8.memory[0x32] = 0xF0;
		chip8.memory[0x33] = 0x90;
		chip8.memory[0x34] = 0xF0;
		chip8.memory[0x35] = 0x90;
		chip8.memory[0x36] = 0x90;

		chip8.memory[0x37] = 0xE0;
		chip8.memory[0x38] = 0x90;
		chip8.memory[0x39] = 0xE0;
		chip8.memory[0x3A] = 0x90;
		chip8.memory[0x3B] = 0xE0;

		chip8.memory[0x3C] = 0xF0;
		chip8.memory[0x3D] = 0x80;
		chip8.memory[0x3E] = 0x80;
		chip8.memory[0x3F] = 0x80;
		chip8.memory[0x40] = 0xF0;

		chip8.memory[0x41] = 0xE0;
		chip8.memory[0x42] = 0x90;
		chip8.memory[0x43] = 0x90;
		chip8.memory[0x44] = 0x90;
		chip8.memory[0x45] = 0xE0;

		chip8.memory[0x46] = 0xF0;
		chip8.memory[0x47] = 0x80;
		chip8.memory[0x48] = 0xF0;
		chip8.memory[0x49] = 0x80;
		chip8.memory[0x4A] = 0xF0;

		chip8.memory[0x4B] = 0xF0;
		chip8.memory[0x4C] = 0x80;
		chip8.memory[0x4D] = 0xF0;
		chip8.memory[0x4E] = 0x80;
		chip8.memory[0x4F] = 0x80;

		chip8
	}

	fn load_rom(&mut self, filename: String) {
		let mut file = File::open(filename).expect("file not found");
		let mut buffer: Vec<u8> = Vec::new();
		file.read_to_end(&mut buffer).expect("something went wrong reading the file");
		self.memory[ROM_START_ADDRESS as usize..ROM_START_ADDRESS as usize + buffer.len()].copy_from_slice(&buffer);
	}

	#[cfg(feature="debugger")]
	fn print_registers(&self) {
		println!("PC= {:x}", self.register_pc);
		println!("I= {:x}", self.register_i);
		println!("DT= {:x}", self.register_dt);
		println!("ST= {:x}", self.register_st);
		println!("SP= {:x}", self.register_sp);
		for i in 0..16 as usize {
			println!("V{}= {:x}", i, self.register_v[i]);
		}
		let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
	}

	pub fn run(&mut self, filename: String) {
		self.load_rom(filename);

		let mut time = Instant::now();

		#[cfg(not(feature="interpreter"))]
		let mut recompiler = Recompiler::new();
		
		loop {
			#[cfg(feature="debugger")]
			self.print_registers();

			#[cfg(feature="interpreter")]
			Interpreter::execute_next_instruction(self);

			#[cfg(not(feature="interpreter"))]
			recompiler.execute_next_code_block(self);

			// ~60Hz
			if time.elapsed() >= Duration::from_millis(1000 / 60) { 
				time = Instant::now();
				self.keyboard.update_key_states();
				self.display.refresh();
				if self.register_dt > 0 {
					self.register_dt -= 1
				}
				if self.register_st > 0 {
					self.register_st -= 1;
					// TODO: beep
				}
			}
		}
	}
}
