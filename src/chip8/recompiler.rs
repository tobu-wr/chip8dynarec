use std::collections::HashMap;

use chip8::Chip8;
use chip8::codeblock::CodeBlock;

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

	fn recompile_next_code_block(chip8: &mut Chip8) -> CodeBlock {
		let mut raw_code: Vec<u8> = Vec::new();

		raw_code.push(0xC3); // RET

		CodeBlock::new(raw_code)
	}
}
