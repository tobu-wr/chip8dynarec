extern crate sdl2;

use std::collections::HashSet;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

pub struct Keyboard {
	events: sdl2::EventPump
}

impl Keyboard {
	pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
		Keyboard {
			events: sdl_context.event_pump().unwrap()
		}
	}

	pub fn update_key_states(&mut self) {
		self.events.pump_events();
	}

	pub extern "stdcall" fn is_pressed(&self, key: u8) -> bool {
		let pressed_keys: HashSet<Keycode> = self.events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
		match key {
			0x0 => pressed_keys.contains(&Keycode::Kp0),
			0x1 => pressed_keys.contains(&Keycode::Kp1),
			0x2 => pressed_keys.contains(&Keycode::Kp2),
			0x3 => pressed_keys.contains(&Keycode::Kp3),
			0x4 => pressed_keys.contains(&Keycode::Kp4),
			0x5 => pressed_keys.contains(&Keycode::Kp5),
			0x6 => pressed_keys.contains(&Keycode::Kp6),
			0x7 => pressed_keys.contains(&Keycode::Kp7),
			0x8 => pressed_keys.contains(&Keycode::Kp8),
			0x9 => pressed_keys.contains(&Keycode::Kp9),
			0xA => pressed_keys.contains(&Keycode::A),
			0xB => pressed_keys.contains(&Keycode::B),
			0xC => pressed_keys.contains(&Keycode::C),
			0xD => pressed_keys.contains(&Keycode::D),
			0xE => pressed_keys.contains(&Keycode::E),
			0xF => pressed_keys.contains(&Keycode::F),
			_ => false
		}
	}

	pub extern "stdcall" fn wait_key_press(&mut self) -> u8 {
		loop {
			for event in self.events.wait_iter() {
				match event {
					Event::KeyDown { keycode: Some(Keycode::Kp0), ..} => return 0x0,
					Event::KeyDown { keycode: Some(Keycode::Kp1), ..} => return 0x1,
					Event::KeyDown { keycode: Some(Keycode::Kp2), ..} => return 0x2,
					Event::KeyDown { keycode: Some(Keycode::Kp3), ..} => return 0x3,
					Event::KeyDown { keycode: Some(Keycode::Kp4), ..} => return 0x4,
					Event::KeyDown { keycode: Some(Keycode::Kp5), ..} => return 0x5,
					Event::KeyDown { keycode: Some(Keycode::Kp6), ..} => return 0x6,
					Event::KeyDown { keycode: Some(Keycode::Kp7), ..} => return 0x7,
					Event::KeyDown { keycode: Some(Keycode::Kp8), ..} => return 0x8,
					Event::KeyDown { keycode: Some(Keycode::Kp9), ..} => return 0x9,
					Event::KeyDown { keycode: Some(Keycode::A), ..} => return 0xA,
					Event::KeyDown { keycode: Some(Keycode::B), ..} => return 0xB,
					Event::KeyDown { keycode: Some(Keycode::C), ..} => return 0xC,
					Event::KeyDown { keycode: Some(Keycode::D), ..} => return 0xD,
					Event::KeyDown { keycode: Some(Keycode::E), ..} => return 0xE,
					Event::KeyDown { keycode: Some(Keycode::F), ..} => return 0xF,
					_ => {}
				}
			}
		}
	}
}
