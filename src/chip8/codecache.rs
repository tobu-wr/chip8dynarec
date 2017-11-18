extern crate libc;

use std::ops::Index;
use std::{ptr, mem};

use chip8::MEMORY_SIZE;
use chip8::ROM_START_ADDRESS;
use chip8::codeblock::CodeBlock;
use chip8::codeemitter::CodeEmitter;

pub struct CodeCache {
	pub x86_block_addresses: [u32; MEMORY_SIZE],
	reserved: [bool; MEMORY_SIZE],
	blocks: [*mut CodeBlock; MEMORY_SIZE]
}

impl CodeCache {
	pub fn new(register_pc: &u16) -> CodeCache {
		let mut code_cache = CodeCache {
			x86_block_addresses: [0; MEMORY_SIZE],
			reserved: [false; MEMORY_SIZE],
			blocks: [ptr::null_mut(); MEMORY_SIZE]
		};

		for i in ROM_START_ADDRESS..MEMORY_SIZE as u16 {
			let mut code_emitter = CodeEmitter::new();
			code_emitter.mov_imm_to_m16(i, register_pc);
			code_emitter.ret();
			unsafe {
				let new_ptr = libc::malloc(mem::size_of::<CodeBlock>()) as *mut CodeBlock;
				ptr::write(new_ptr, CodeBlock::new(code_emitter.raw_code));
				code_cache.blocks[i as usize] = new_ptr;
			}
			code_cache.x86_block_addresses[i as usize] = code_cache[i].get_x86address();
		}

		code_cache
	}

	pub fn contains_block(&self, address: u16) -> bool {
		self.reserved[address as usize]
	}

	pub fn insert(&mut self, address: u16, block: CodeBlock) {
		unsafe { 
			ptr::drop_in_place(self.blocks[address as usize]);
			ptr::write(self.blocks[address as usize], block);
		}
		self.reserved[address as usize] = true;
		self.x86_block_addresses[address as usize] = self[address].get_x86address();
	}
}

impl Index<u16> for CodeCache {
	type Output = CodeBlock;

	fn index(&self, address: u16) -> &CodeBlock {
		unsafe { self.blocks[address as usize].as_ref().unwrap() }
    }
}

impl Drop for CodeCache {
	fn drop(&mut self) {
		for iter in self.blocks.iter() {
			if !(*iter).is_null() {
				unsafe { 
					ptr::drop_in_place(*iter);
					libc::free(*iter as *mut libc::c_void);
				}
			}
		}
	}
}
