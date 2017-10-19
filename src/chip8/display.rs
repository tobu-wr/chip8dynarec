extern crate sdl2;

use self::sdl2::video::Window;
use self::sdl2::render::Canvas;
use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SCALE: u32 = 8;

pub struct Display {
	frame_buffer: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
	canvas: Canvas<Window>
}

impl Display {
	pub fn new(sdl_context: &sdl2::Sdl) -> Display {
		let video_subsystem = sdl_context.video().unwrap();
		let window = video_subsystem.window("chip8dynarec", DISPLAY_WIDTH as u32 * DISPLAY_SCALE, DISPLAY_HEIGHT as u32 * DISPLAY_SCALE)
    								.position_centered()
    								.opengl()
    								.build()
    								.unwrap();

		Display {
			frame_buffer: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
			canvas: window.into_canvas().build().unwrap()
		}
	}

	pub fn clear(&mut self) {
		for y in 0..DISPLAY_HEIGHT {
			for x in 0..DISPLAY_WIDTH {
				self.frame_buffer[y * DISPLAY_WIDTH + x] = 0;
			}
		}
	}

	pub fn draw_sprite(&mut self, x_position: u8, mut y_position: u8, sprite: &[u8]) -> bool {
		let mut pixel_erased = false;
		y_position %= DISPLAY_HEIGHT as u8;

		for byte in sprite {
			for i in 0..8 {
				let offset = y_position as usize * DISPLAY_WIDTH + (x_position as usize + i) % DISPLAY_WIDTH;
				let old_pixel = self.frame_buffer[offset];

				self.frame_buffer[offset] ^= (byte >> (7 - i)) & 1;
				
				if old_pixel == 1 && self.frame_buffer[offset] == 0 {
					pixel_erased = true;
				}
			}

			y_position = (y_position + 1) % DISPLAY_HEIGHT as u8;
		}

		pixel_erased
	}

	pub fn refresh(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0x3F, 0x51, 0xB5));
		self.canvas.clear();
		self.canvas.set_draw_color(Color::RGB(0xC5, 0xCA, 0xE9));

		for y in 0..DISPLAY_HEIGHT {
			for x in 0..DISPLAY_WIDTH {
				if self.frame_buffer[y * DISPLAY_WIDTH + x] == 1 {
					let pixel = Rect::new((x * DISPLAY_SCALE as usize) as i32, (y * DISPLAY_SCALE as usize) as i32, DISPLAY_SCALE, DISPLAY_SCALE);
					let _ = self.canvas.fill_rect(pixel);
				}
			}
		}

		self.canvas.present();
	}
}
