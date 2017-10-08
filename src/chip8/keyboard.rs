extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

const KEY_COUNT: usize = 16;

pub struct Keyboard {
	events: sdl2::EventPump,
	key_states: [bool; KEY_COUNT]
}

impl Keyboard {
	pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
		Keyboard {
			events: sdl_context.event_pump().unwrap(),
			key_states: [false; KEY_COUNT]
		}
	}

	fn update_key_states(&mut self) {
		for event in self.events.poll_iter() {
			match event {
				Event::KeyDown { keycode: Some(Keycode::Num0), ..} => self.key_states[0x0] = true,
				Event::KeyDown { keycode: Some(Keycode::Num1), ..} => self.key_states[0x1] = true,
				Event::KeyDown { keycode: Some(Keycode::Num2), ..} => self.key_states[0x2] = true,
				Event::KeyDown { keycode: Some(Keycode::Num3), ..} => self.key_states[0x3] = true,
				Event::KeyDown { keycode: Some(Keycode::Num4), ..} => self.key_states[0x4] = true,
				Event::KeyDown { keycode: Some(Keycode::Num5), ..} => self.key_states[0x5] = true,
				Event::KeyDown { keycode: Some(Keycode::Num6), ..} => self.key_states[0x6] = true,
				Event::KeyDown { keycode: Some(Keycode::Num7), ..} => self.key_states[0x7] = true,
				Event::KeyDown { keycode: Some(Keycode::Num8), ..} => self.key_states[0x8] = true,
				Event::KeyDown { keycode: Some(Keycode::Num9), ..} => self.key_states[0x9] = true,
				Event::KeyDown { keycode: Some(Keycode::A), ..} => self.key_states[0xA] = true,
				Event::KeyDown { keycode: Some(Keycode::B), ..} => self.key_states[0xB] = true,
				Event::KeyDown { keycode: Some(Keycode::C), ..} => self.key_states[0xC] = true,
				Event::KeyDown { keycode: Some(Keycode::D), ..} => self.key_states[0xD] = true,
				Event::KeyDown { keycode: Some(Keycode::E), ..} => self.key_states[0xE] = true,
				Event::KeyDown { keycode: Some(Keycode::F), ..} => self.key_states[0xF] = true,
				Event::KeyUp { keycode: Some(Keycode::Num0), ..} => self.key_states[0x0] = false,
				Event::KeyUp { keycode: Some(Keycode::Num1), ..} => self.key_states[0x1] = false,
				Event::KeyUp { keycode: Some(Keycode::Num2), ..} => self.key_states[0x2] = false,
				Event::KeyUp { keycode: Some(Keycode::Num3), ..} => self.key_states[0x3] = false,
				Event::KeyUp { keycode: Some(Keycode::Num4), ..} => self.key_states[0x4] = false,
				Event::KeyUp { keycode: Some(Keycode::Num5), ..} => self.key_states[0x5] = false,
				Event::KeyUp { keycode: Some(Keycode::Num6), ..} => self.key_states[0x6] = false,
				Event::KeyUp { keycode: Some(Keycode::Num7), ..} => self.key_states[0x7] = false,
				Event::KeyUp { keycode: Some(Keycode::Num8), ..} => self.key_states[0x8] = false,
				Event::KeyUp { keycode: Some(Keycode::Num9), ..} => self.key_states[0x9] = false,
				Event::KeyUp { keycode: Some(Keycode::A), ..} => self.key_states[0xA] = false,
				Event::KeyUp { keycode: Some(Keycode::B), ..} => self.key_states[0xB] = false,
				Event::KeyUp { keycode: Some(Keycode::C), ..} => self.key_states[0xC] = false,
				Event::KeyUp { keycode: Some(Keycode::D), ..} => self.key_states[0xD] = false,
				Event::KeyUp { keycode: Some(Keycode::E), ..} => self.key_states[0xE] = false,
				Event::KeyUp { keycode: Some(Keycode::F), ..} => self.key_states[0xF] = false,
				_ => {}
			}
		}
	}

	pub fn is_pressed(&mut self, key: u8) -> bool {
		self.update_key_states();
		self.key_states[key as usize]
	}

	pub fn wait_key_press(&mut self) -> u8 {
		self.update_key_states();

		// wait for a key press
		loop {
			for event in self.events.poll_iter() {
				match event {
					Event::KeyDown { keycode: Some(Keycode::Num0), ..} => {
						self.key_states[0x0] = true;
						return 0x0
					},
					Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {
						self.key_states[0x1] = true;
						return 0x1
					},
					Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {
						self.key_states[0x2] = true;
						return 0x2
					},
					Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {
						self.key_states[0x3] = true;
						return 0x3
					},
					Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {
						self.key_states[0x4] = true;
						return 0x4
					},
					Event::KeyDown { keycode: Some(Keycode::Num5), ..} => {
						self.key_states[0x5] = true;
						return 0x5
					},
					Event::KeyDown { keycode: Some(Keycode::Num6), ..} => {
						self.key_states[0x6] = true;
						return 0x6
					},
					Event::KeyDown { keycode: Some(Keycode::Num7), ..} => {
						self.key_states[0x7] = true;
						return 0x7
					},
					Event::KeyDown { keycode: Some(Keycode::Num8), ..} => {
						self.key_states[0x8] = true;
						return 0x8
					},
					Event::KeyDown { keycode: Some(Keycode::Num9), ..} => {
						self.key_states[0x9] = true;
						return 0x9
					},
					Event::KeyDown { keycode: Some(Keycode::A), ..} => {
						self.key_states[0xA] = true;
						return 0xA
					},
					Event::KeyDown { keycode: Some(Keycode::B), ..} => {
						self.key_states[0xB] = true;
						return 0xB
					},
					Event::KeyDown { keycode: Some(Keycode::C), ..} => {
						self.key_states[0xC] = true;
						return 0xC
					},
					Event::KeyDown { keycode: Some(Keycode::D), ..} => {
						self.key_states[0xD] = true;
						return 0xD
					},
					Event::KeyDown { keycode: Some(Keycode::E), ..} => {
						self.key_states[0xE] = true;
						return 0xE
					},
					Event::KeyDown { keycode: Some(Keycode::F), ..} => {
						self.key_states[0xF] = true;
						return 0xF
					},
					Event::KeyUp { keycode: Some(Keycode::Num0), ..} => self.key_states[0x0] = false,
					Event::KeyUp { keycode: Some(Keycode::Num1), ..} => self.key_states[0x1] = false,
					Event::KeyUp { keycode: Some(Keycode::Num2), ..} => self.key_states[0x2] = false,
					Event::KeyUp { keycode: Some(Keycode::Num3), ..} => self.key_states[0x3] = false,
					Event::KeyUp { keycode: Some(Keycode::Num4), ..} => self.key_states[0x4] = false,
					Event::KeyUp { keycode: Some(Keycode::Num5), ..} => self.key_states[0x5] = false,
					Event::KeyUp { keycode: Some(Keycode::Num6), ..} => self.key_states[0x6] = false,
					Event::KeyUp { keycode: Some(Keycode::Num7), ..} => self.key_states[0x7] = false,
					Event::KeyUp { keycode: Some(Keycode::Num8), ..} => self.key_states[0x8] = false,
					Event::KeyUp { keycode: Some(Keycode::Num9), ..} => self.key_states[0x9] = false,
					Event::KeyUp { keycode: Some(Keycode::A), ..} => self.key_states[0xA] = false,
					Event::KeyUp { keycode: Some(Keycode::B), ..} => self.key_states[0xB] = false,
					Event::KeyUp { keycode: Some(Keycode::C), ..} => self.key_states[0xC] = false,
					Event::KeyUp { keycode: Some(Keycode::D), ..} => self.key_states[0xD] = false,
					Event::KeyUp { keycode: Some(Keycode::E), ..} => self.key_states[0xE] = false,
					Event::KeyUp { keycode: Some(Keycode::F), ..} => self.key_states[0xF] = false,
					_ => {}
				}
			}
		}
	}
}
