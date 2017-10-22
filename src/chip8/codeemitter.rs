pub struct CodeEmitter {
	pub raw_code: Vec<u8>
}

impl CodeEmitter {
	pub fn new() -> CodeEmitter {
		CodeEmitter {
			raw_code: Vec::new()
		}
	}

	pub fn add_imm_to_m8(&mut self, imm: u8, disp: u32) {
		self.raw_code.push(0x80);
		self.raw_code.push(0x05);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
	}

	pub fn add_imm_to_m16(&mut self, imm: u16, disp: u32) {
		self.raw_code.push(0x66);
		self.raw_code.push(0x81);
		self.raw_code.push(0x05);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
	}

	pub fn cmp_imm_with_m8(&mut self, imm: u8, disp: u32) {
		self.raw_code.push(0x80);
		self.raw_code.push(0x3D);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm);
	}

	pub fn jne(&mut self, disp: u8) {
		self.raw_code.push(0x75);
		self.raw_code.push(disp);
	}

	pub fn mov_imm_to_al(&mut self, imm: u8) {
		self.raw_code.push(0xB0);
		self.raw_code.push(imm);
	}

	pub fn mov_ax_to_m(&mut self, disp: u32) {
		self.raw_code.push(0x66);
		self.raw_code.push(0xA3);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn mov_imm_to_m8(&mut self, imm: u8, disp: u32) {
		self.raw_code.push(0xC6);
		self.raw_code.push(0x05);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm);
	}

	pub fn mov_imm_to_m16(&mut self, imm: u16, disp: u32) {
		self.raw_code.push(0x66);
		self.raw_code.push(0xC7);
		self.raw_code.push(0x05);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
	}

	pub fn mul_al_with_m(&mut self, disp: u32) {
		self.raw_code.push(0xF6);
		self.raw_code.push(0x25);
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn ret(&mut self) {
		self.raw_code.push(0xC3);
	}
}
