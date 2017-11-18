extern crate memmap;

use std::mem;
use self::memmap::{Mmap, Protection};

pub struct CodeBlock {
	executable_code: Mmap,
	function_pointer: fn()
}

impl CodeBlock {
	pub fn new(raw_code: Vec<u8>) -> CodeBlock {
		let mut code_block = CodeBlock {
			executable_code: Mmap::anonymous(raw_code.len(), Protection::ReadWrite).unwrap(),
			function_pointer: ||{}
		};

		unsafe { 
			code_block.executable_code.as_mut_slice().copy_from_slice(&raw_code);
			let _ = code_block.executable_code.set_protection(Protection::ReadExecute);
			code_block.function_pointer = mem::transmute(code_block.executable_code.mut_ptr());
		}
		
		code_block
	}

	pub fn get_x86address(&self) -> u32 {
		self.function_pointer as u32
	}

	pub fn execute(&self) {
		(self.function_pointer)();
	}
}
