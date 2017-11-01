extern crate libc;

use std::ops::Index;
use std::{ptr, mem};

use chip8::MEMORY_SIZE;
use chip8::codeblock::CodeBlock;

pub struct CodeCache {
	code_blocks: [*mut CodeBlock; MEMORY_SIZE]
}

impl CodeCache {
	pub fn new() -> CodeCache {
		CodeCache {
			code_blocks: [ptr::null_mut(); MEMORY_SIZE]
		}
	}

	pub fn contains_address(&self, address: u16) -> bool {
		!self.code_blocks[address as usize].is_null()
	}

	pub fn insert(&mut self, address: u16, code_block: CodeBlock) {
		unsafe {
			let new_ptr = libc::malloc(mem::size_of::<CodeBlock>()) as *mut CodeBlock;
			ptr::write(new_ptr, code_block);
			self.code_blocks[address as usize] = new_ptr;
		}
	}
}

impl Index<u16> for CodeCache {
	type Output = CodeBlock;

	fn index(&self, address: u16) -> &CodeBlock {
		unsafe { self.code_blocks[address as usize].as_ref().unwrap() }
    }
}
