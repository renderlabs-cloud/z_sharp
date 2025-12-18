// This file will be removed in the future.

use ::mlua::prelude::*;
use ::mlua::{
	FromLua,
	// UserData,
	// UserDataRef,
	IntoLua,
	Table,
	Value,
};

use ::serde::{Deserialize, Serialize};

use ::std::{cell::RefCell, rc::Rc};

use ::derive_more::with_trait::Deref;

#[derive(Clone, Deref, Debug)]
pub struct Glue(#[deref] Lua);

impl Default for Glue {
	fn default() -> Self {
		return Self(Lua::default());
	}
}

#[derive(Clone, Default, Debug)]
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

	fn deref(&self) -> &Self::Target {
		return &(self.glue);
	}
}

impl IntoLua for Gluea {
	fn into_lua(self, lua: &Lua) -> ::mlua::Result<Value> {
		let gluea_table: Table = lua.create_table()?;

		let metatable: Table = lua.create_table()?;

		gluea_table.set_metatable(Some(metatable))?;

		return Ok(Value::Table(gluea_table));
	}
}

impl FromLua for Gluea {
	/// This function does not return the Glue instance!
	fn from_lua(_: Value, _: &Lua) -> ::mlua::Result<Self> {
		return Ok(Gluea::default());
	}
}

// TODO: Implement in mlua-magic so this isn't needed.
#[derive(Clone, Deref, Deserialize, Debug)]
pub struct LuaHider<T>(#[deref] pub(self) Option<T>);

impl<T> LuaHider<T> {
	pub fn new(value: T) -> Self {
		return Self(Some(value));
	}

	pub fn peek(&self) -> ::mlua::Result<&T> {
		match &(self.0) {
			| Some(value) => {
				return Ok(value);
			},
			| None => {
				todo!();
			},
		};
	}
}

impl<T: IntoLua> IntoLua for LuaHider<T> {
	fn into_lua(self, _: &Lua) -> ::mlua::Result<Value> {
		return Ok(Value::Nil);
	}
}

impl<T: FromLua> FromLua for LuaHider<T> {
	fn from_lua(_: Value, _: &Lua) -> ::mlua::Result<Self> {
		return Ok(Self(None));
	}
}

// Lua Rc
#[derive(Clone, Deref, Debug)]
pub struct LuaRc<T>(#[deref] pub(self) Rc<T>);

impl<T> LuaRc<T> {
	pub fn new(value: T) -> Self {
		return Self(Rc::new(value));
	}
}

impl<T: IntoLua> IntoLua for LuaRc<T> {
	fn into_lua(self, _: &Lua) -> ::mlua::Result<Value> {
		return Ok(Value::Nil);
	}
}

impl<T: FromLua> FromLua for LuaRc<T> {
	fn from_lua(value: Value, lua: &Lua) -> ::mlua::Result<Self> {
		return Ok(Self(Rc::new(T::from_lua(value, lua)?)));
	}
}
