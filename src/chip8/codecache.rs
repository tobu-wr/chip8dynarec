use std::ops::Index;
use std::ptr;

use chip8::MEMORY_SIZE;
use chip8::codeblock::CodeBlock;

pub struct CodeCache {
	code_blocks: Vec<CodeBlock>,
	map: [*const CodeBlock; MEMORY_SIZE]
}

impl CodeCache {
	pub fn new() -> CodeCache {
		CodeCache {
			code_blocks: Vec::new(),
			map: [ptr::null(); MEMORY_SIZE]
		}
	}

	pub fn contains_address(&self, address: u16) -> bool {
		!self.map[address as usize].is_null()
	}

	pub fn insert(&mut self, address: u16, code_block: CodeBlock) {
		self.code_blocks.push(code_block);
		self.map[address as usize] = self.code_blocks.last().unwrap();
	}
}

impl Index<u16> for CodeCache {
	type Output = CodeBlock;

	fn index(&self, address: u16) -> &CodeBlock {
        unsafe { &*self.map[address as usize] }
    }
}
