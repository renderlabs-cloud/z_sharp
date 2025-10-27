pub mod lua;
pub mod capture;
pub mod consumer;

use std::collections::HashMap;

use ::mlua::prelude::*;

use ::nestify::nest;

pub use crate::modification;

nest! {
	pub struct Modification {
		pub name: &'static str,
		
		pub ctx: Lua,
		pub exports: HashMap<String, mlua::Value>
	}
}