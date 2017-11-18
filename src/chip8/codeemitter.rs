pub struct CodeEmitter {
	pub raw_code: Vec<u8>
}

impl CodeEmitter {
	pub fn new() -> CodeEmitter {
		CodeEmitter {
			raw_code: Vec::new()
		}
	}

	fn push_u8(&mut self, value: u8) {
		self.raw_code.push(value);
	}

	fn push_u16(&mut self, value: u16) {
		self.push_u8(value as u8);
		self.push_u8((value >> 8) as u8)
	}

	fn push_u32(&mut self, value: u32) {
		self.push_u8(value as u8);
		self.push_u8((value >> 8) as u8);
		self.push_u8((value >> 16) as u8);
		self.push_u8((value >> 24) as u8);
	}

	pub fn add_al_to_al(&mut self) {
		self.push_u8(0x00);
		self.push_u8(0xC0);
	}

	pub fn add_cx_to_ax(&mut self) {
		self.push_u8(0x66);
		self.push_u8(0x01);
		self.push_u8(0xC8);
	}

	pub fn add_imm_to_ax(&mut self, imm: u16) {
		self.push_u8(0x66);
		self.push_u8(0x05);
		self.push_u16(imm);
	}

	pub fn add_imm_to_eax(&mut self, imm: u32) {
		self.push_u8(0x05);
		self.push_u32(imm);
	}

	pub fn add_ax_to_m(&mut self, m: &u16) {
		self.push_u8(0x66);
		self.push_u8(0x01);
		self.push_u8(0x05);
		self.push_u32(m as *const u16 as u32);
	}

	pub fn add_imm_to_m8(&mut self, imm: u8, m: &u8) {
		self.push_u8(0x80);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
		self.push_u8(imm);
	}

	pub fn add_imm_to_m16(&mut self, imm: u16, m: &u16) {
		self.push_u8(0x66);
		self.push_u8(0x81);
		self.push_u8(0x05);
		self.push_u32(m as *const u16 as u32);
		self.push_u16(imm);
	}

	pub fn and_al_imm(&mut self, imm: u8) {
		self.push_u8(0x24);
		self.push_u8(imm);
	}

	pub fn and_cl_imm(&mut self, imm: u8) {
		self.push_u8(0x80);
		self.push_u8(0xE1);
		self.push_u8(imm);
	}

	pub fn and_m_al(&mut self, m: &u8) {
		self.push_u8(0x20);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn call_eax(&mut self) {
		self.push_u8(0xFF);
		self.push_u8(0xD0);
	}

	pub fn cmp_al_with_imm(&mut self, imm: u8) {
		self.push_u8(0x3C);
		self.push_u8(imm);
	}

	pub fn cmp_ax_with_imm(&mut self, imm: u16) {
		self.push_u8(0x66);
		self.push_u8(0x3D);
		self.push_u16(imm);
	}

	pub fn cmp_m_with_al(&mut self, m: &u8) {
		self.push_u8(0x3A);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn cmp_imm_with_m8(&mut self, imm: u8, m: &u8) {
		self.push_u8(0x80);
		self.push_u8(0x3D);
		self.push_u32(m as *const u8 as u32);
		self.push_u8(imm);
	}

	pub fn div_dl(&mut self) {
		self.push_u8(0xF6);
		self.push_u8(0xF2);
	}

	pub fn je(&mut self, disp: i8) {
		self.push_u8(0x74);
		self.push_u8(disp as u8);
	}

	pub fn jne(&mut self, disp: i8) {
		self.push_u8(0x75);
		self.push_u8(disp as u8);
	}

	pub fn jmp_eax(&mut self) {
		self.push_u8(0xFF);
		self.push_u8(0xE0);
	}

	pub fn mov_al_to_cl(&mut self) {
		self.push_u8(0x88);
		self.push_u8(0xC1);
	}

	pub fn mov_imm_to_al(&mut self, imm: u8) {
		self.push_u8(0xB0);
		self.push_u8(imm);
	}

	pub fn mov_imm_to_dl(&mut self, imm: u8) {
		self.push_u8(0xB2);
		self.push_u8(imm);
	}

	pub fn mov_imm_to_edi(&mut self, imm: u32) {
		self.push_u8(0xBF);
		self.push_u32(imm);
	}

	pub fn mov_imm_to_eax(&mut self, imm: u32) {
		self.push_u8(0xB8);
		self.push_u32(imm);
	}

	pub fn mov_cl_to_m(&mut self, m: &u8) {
		self.push_u8(0x88);
		self.push_u8(0x0D);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn mov_al_to_m(&mut self, m: &u8) {
		self.push_u8(0xA2);
		self.push_u32(m as *const u8 as u32);
	}

	// mov byte ptr [edi+ecx],ah
	pub fn mov_ah_to_m_ediecx(&mut self) {
		self.push_u8(0x88);
		self.push_u8(0x24);
		self.push_u8(0x0F);
	}

	// mov byte ptr [edi+ecx],al
	pub fn mov_al_to_m_ediecx(&mut self) {
		self.push_u8(0x88);
		self.push_u8(0x04);
		self.push_u8(0x0F);
	}

	pub fn mov_ax_to_m(&mut self, m: &u16) {
		self.push_u8(0x66);
		self.push_u8(0xA3);
		self.push_u32(m as *const u16 as u32);
	}

	pub fn mov_m_to_al(&mut self, m: &u8) {
		self.push_u8(0xA0);
		self.push_u32(m as *const u8 as u32);
	}

	// mov al,byte ptr [edi+ecx]
	pub fn mov_m_to_al_ediecx(&mut self) {
		self.push_u8(0x8A);
		self.push_u8(0x04);
		self.push_u8(0x0F);
	}

	// mov ax,word ptr [edi+ecx*2]
	pub fn mov_m_to_ax_edi2ecx(&mut self) {
		self.push_u8(0x66);
		self.push_u8(0x8B);
		self.push_u8(0x04);
		self.push_u8(0x4F);
	}

	pub fn mov_m_to_eax(&mut self, m: &u32) {
		self.push_u8(0xA1);
		self.push_u32(m as *const u32 as u32);
	}

	// mov eax,dword ptr [edi+4*ecx]
	pub fn mov_m_to_eax_edi4ecx(&mut self) {
		self.push_u8(0x8B);
		self.push_u8(0x04);
		self.push_u8(0x8F);
	}

	pub fn mov_imm_to_m8(&mut self, imm: u8, m: &u8) {
		self.push_u8(0xC6);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
		self.push_u8(imm);
	}

	pub fn mov_imm_to_m16(&mut self, imm: u16, m: &u16) {
		self.push_u8(0x66);
		self.push_u8(0xC7);
		self.push_u8(0x05);
		self.push_u32(m as *const u16 as u32);
		self.push_u16(imm);
	}

	// mov word ptr [edi+2*ecx],imm16
	pub fn mov_imm_to_m16_edi2ecx(&mut self, imm: u16) {
		self.push_u8(0x66);
		self.push_u8(0xC7);
		self.push_u8(0x04);
		self.push_u8(0x4F);
		self.push_u16(imm);
	}

	pub fn movzx_ah_to_ax(&mut self) {
		self.push_u8(0x66);
		self.push_u8(0x0F);
		self.push_u8(0xB6);
		self.push_u8(0xC4);
	}

	pub fn movzx_m_to_ax(&mut self, m: &u8) {
		self.push_u8(0x66);
		self.push_u8(0x0F);
		self.push_u8(0xB6);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn movzx_m8_to_eax(&mut self, m: &u8) {
		self.push_u8(0x0F);
		self.push_u8(0xB6);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn movzx_m16_to_eax(&mut self, m: &u16) {
		self.push_u8(0x0F);
		self.push_u8(0xB7);
		self.push_u8(0x05);
		self.push_u32(m as *const u16 as u32);
	}

	pub fn movzx_m_to_cx(&mut self, m: &u8) {
		self.push_u8(0x66);
		self.push_u8(0x0F);
		self.push_u8(0xB6);
		self.push_u8(0x0D);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn movzx_m8_to_ecx(&mut self, m: &u8) {
		self.push_u8(0x0F);
		self.push_u8(0xB6);
		self.push_u8(0x0D);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn movzx_m16_to_ecx(&mut self, m: &u16) {
		self.push_u8(0x0F);
		self.push_u8(0xB7);
		self.push_u8(0x0D);
		self.push_u32(m as *const u16 as u32);
	}

	pub fn mul_m8(&mut self, m: &u8) {
		self.push_u8(0xF6);
		self.push_u8(0x25);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn or_m_al(&mut self, m: &u8) {
		self.push_u8(0x08);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn push_eax(&mut self) {
		self.push_u8(0x50);
	}

	pub fn push_imm32(&mut self, imm: u32) {
		self.push_u8(0x68);
		self.push_u32(imm);
	}

	pub fn rdrand_ax(&mut self) {
		self.push_u8(0x66);
		self.push_u8(0x0F);
		self.push_u8(0xC7);
		self.push_u8(0xF0);
	}

	pub fn ret(&mut self) {
		self.push_u8(0xC3);
	}

	pub fn seta_m(&mut self, m: &u8) {
		self.push_u8(0x0F);
		self.push_u8(0x97);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn setae_m(&mut self, m: &u8) {
		self.push_u8(0x0F);
		self.push_u8(0x93);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn shr_al(&mut self) {
		self.push_u8(0xD0);
		self.push_u8(0xE8);
	}

	pub fn shr_al_imm(&mut self, imm: u8) {
		self.push_u8(0xC0);
		self.push_u8(0xE8);
		self.push_u8(imm);
	}

	pub fn sub_m_to_al(&mut self, m: &u8) {
		self.push_u8(0x2A);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}

	pub fn sub_imm_to_m8(&mut self, imm: u8, m: &u8) {
		self.push_u8(0x80);
		self.push_u8(0x2D);
		self.push_u32(m as *const u8 as u32);
		self.push_u8(imm);
	}

	pub fn xor_m_al(&mut self, m: &u8) {
		self.push_u8(0x30);
		self.push_u8(0x05);
		self.push_u32(m as *const u8 as u32);
	}
}
