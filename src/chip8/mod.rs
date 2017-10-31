mod chip8;
mod keyboard;
mod display;

#[cfg(feature="interpreter")]
mod interpreter;

#[cfg(not(feature="interpreter"))]
mod recompiler;

#[cfg(not(feature="interpreter"))]
mod codeblock;

#[cfg(not(feature="interpreter"))]
mod codeemitter;

#[cfg(not(feature="interpreter"))]
mod codecache;

pub use self::chip8::Chip8;

const MEMORY_SIZE: usize = 0x1000;
