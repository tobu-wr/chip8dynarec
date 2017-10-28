pub struct CodeEmitter {
	pub raw_code: Vec<u8>
}

impl CodeEmitter {
	pub fn new() -> CodeEmitter {
		CodeEmitter {
			raw_code: Vec::new()
		}
	}

	pub fn add_imm_to_m8(&mut self, imm: u8, m: *const u8) {
		self.raw_code.push(0x80);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
	}

	pub fn add_imm_to_m16(&mut self, imm: u16, m: *const u16) {
		self.raw_code.push(0x66);
		self.raw_code.push(0x81);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
	}

	pub fn cmp_m_with_al(&mut self, m: *const u8) {
		self.raw_code.push(0x3A);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn cmp_imm_with_m8(&mut self, imm: u8, m: *const u8) {
		self.raw_code.push(0x80);
		self.raw_code.push(0x3D);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm);
	}

	pub fn je(&mut self, disp: u8) {
		self.raw_code.push(0x74);
		self.raw_code.push(disp);
	}

	pub fn jne(&mut self, disp: u8) {
		self.raw_code.push(0x75);
		self.raw_code.push(disp);
	}

	pub fn mov_imm_to_al(&mut self, imm: u8) {
		self.raw_code.push(0xB0);
		self.raw_code.push(imm);
	}

	pub fn mov_imm_to_edi(&mut self, imm: u32) {
		self.raw_code.push(0xBF);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
		self.raw_code.push((imm >> 16) as u8);
		self.raw_code.push((imm >> 24) as u8);
	}

	pub fn mov_al_to_m(&mut self, m: *const u8) {
		self.raw_code.push(0xA2);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn mov_ax_to_m(&mut self, m: *const u16) {
		self.raw_code.push(0x66);
		self.raw_code.push(0xA3);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn mov_m_to_al(&mut self, m: *const u8) {
		self.raw_code.push(0xA0);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	// mov ax,word ptr [edi+esi*2]
	pub fn mov_m16_to_ax_edi2esi(&mut self) {
		self.raw_code.push(0x66);
		self.raw_code.push(0x8B);
		self.raw_code.push(0x04);
		self.raw_code.push(0x77);
	}

	pub fn mov_imm_to_m8(&mut self, imm: u8, m: *const u8) {
		self.raw_code.push(0xC6);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm);
	}

	pub fn mov_imm_to_m16(&mut self, imm: u16, m: *const u16) {
		self.raw_code.push(0x66);
		self.raw_code.push(0xC7);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
	}

	// mov word ptr [edi+2*esi],imm16
	pub fn mov_imm_to_m16_edi2esi(&mut self, imm: u16) {
		self.raw_code.push(0x66);
		self.raw_code.push(0xC7);
		self.raw_code.push(0x04);
		self.raw_code.push(0x77);
		self.raw_code.push(imm as u8);
		self.raw_code.push((imm >> 8) as u8);
	}

	pub fn movsx_m8_to_esi(&mut self, m: *const u8) {
		self.raw_code.push(0x0F);
		self.raw_code.push(0xBE);
		self.raw_code.push(0x35);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn mul_m8(&mut self, m: *const u8) {
		self.raw_code.push(0xF6);
		self.raw_code.push(0x25);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn ret(&mut self) {
		self.raw_code.push(0xC3);
	}

	pub fn setae_m(&mut self, m: *const u8) {
		self.raw_code.push(0x0F);
		self.raw_code.push(0x93);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn sub_m_to_al(&mut self, m: *const u8) {
		self.raw_code.push(0x2A);
		self.raw_code.push(0x05);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
	}

	pub fn sub_imm_to_m8(&mut self, imm: u8, m: *const u8) {
		self.raw_code.push(0x80);
		self.raw_code.push(0x2D);
		let disp = m as u32;
		self.raw_code.push(disp as u8);
		self.raw_code.push((disp >> 8) as u8);
		self.raw_code.push((disp >> 16) as u8);
		self.raw_code.push((disp >> 24) as u8);
		self.raw_code.push(imm as u8);
	}
}
