pub mod gluea;
pub mod lua;

use ::mlua::prelude::*;

use ::nestify::nest;

pub use crate::modification;

nest! {
	pub struct Modification {
		pub name: &'static str,
		pub lua: Lua,

	}
}
