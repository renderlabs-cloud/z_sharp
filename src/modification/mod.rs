pub mod lua;
pub mod capture;
pub mod consumer;
pub mod gluea;

use ::mlua::prelude::*;

use ::nestify::nest;

pub use crate::modification;

nest! {
	pub struct Modification {
		pub name: &'static str,
		pub lua: Lua,
		
	}
}