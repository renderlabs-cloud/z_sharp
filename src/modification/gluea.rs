use ::mlua::{
	Value, Table,
	// UserData, MaybeSend,
	IntoLua, // 
	// IntoLuaMulti,
	FromLua, // FromLuaMulti,
//	MultiValue,
//	Error,
};
use ::mlua::prelude::*;

// use ::std::rc::{ Rc, };
use ::std::cell::{ RefCell, };

use ::derive_more::{
	with_trait::{
		Deref,
		
	},
};

#[derive(Clone, Deref, Debug)]
pub struct Glue(#[deref] Lua);

impl Default for Glue {
	fn default() -> Self {
		return Self(Lua::default());
	}
}

#[derive(Default, Clone, Debug)]
pub struct Gluea {
	pub(self) glue: RefCell<Glue>,
}

impl Gluea {
	pub fn new(lua: Lua) -> Self {
		return Self {
			glue: RefCell::new(Glue(lua)),
		};
	}
}

impl Deref for Gluea {
	type Target = RefCell<Glue>;

	fn deref(& self) -> & Self::Target {
		return &(self.glue);
	}
}

impl IntoLua for Gluea {
	fn into_lua(self, lua: & Lua) -> mlua::Result<Value> {
		let gluea_table: Table = lua.create_table()?;

		let metatable: Table = lua.create_table()?;

		metatable.set("__tostring", lua.create_function(|_: & Lua, (): ()| {
			return Ok("Gluea");
		})?)?;

		gluea_table.set_metatable(Some(metatable))?;

		return Ok(Value::Table(gluea_table));
	}
}

impl FromLua for Gluea {
	/// This function does not return the Glue instance!
	fn from_lua(_: Value, _: & Lua) -> mlua::Result<Self> {
		return Ok(Gluea::default());
	}
}

// TODO: Implement for Box so this isn't needed.
#[derive(Clone, Deref, Debug)]
pub struct LuaBox<T>(#[deref] pub Box<T>);

impl<T: IntoLua> IntoLua for LuaBox<T> {
	fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
		// Just forward to the inner type.
		return T::into_lua(*(self.0), lua);
	}
}

impl<T: FromLua> FromLua for LuaBox<T>{
	fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {		
		return Ok(
			Self(
				Box::new(
					T::from_lua(value, lua)?
				)
			)
		);
	}
}