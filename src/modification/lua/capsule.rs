#[rustfmt::skip]
use ::mlua::{
	Lua, WeakLua,
	Value, Table,
	ObjectLike,
	FromLua, IntoLua,
	AnyUserData,
};

use ::std::{marker::PhantomData};

use ::rust_i18n::t;

use ::derive_more::{Debug};

#[derive(Clone, Debug)]
pub(crate) struct Tour {
	#[debug(skip)]
	pub(self) owner: WeakLua,
	pub(self) stable: bool,
}

impl Tour {
	fn new(owner: WeakLua) -> Self {
		return Self {
			owner: owner,
			stable: true,
		};
	}

	fn invalidate(&mut self) -> () {
		self.stable = false;
	}
}

#[derive(Clone, Debug)]
pub struct Capsule<T> {
	pub(self) tour: Tour,
	pub(self) table: Option<Table>,
	pub(self) phantom: PhantomData<T>,
}

impl<T> Capsule<T> {
	/// Enscapulate a [`Table`] and the [`Lua`] instance that created it.
	/// To get the [`Lua`] instance and the [`Table`], use [`Capsule::extract`].
	pub fn new(table: Table) -> Self {
		let weak_lua: WeakLua = table.weak_lua().clone();

		return Self {
			tour: Tour::new(weak_lua),
			table: Some(table),
			phantom: PhantomData,
		};
	}

	/// Returns the [`Lua`] instance and the provided [`Table`].
	/// **Do not use this function to get the [`Lua`] instance from a different thread. [`Lua`] is not thread-safe!**
	pub(crate) fn extract(&self) -> (Lua, Table) {
		if !self.tour.stable {
			// It is ok to panic here, as we can't do anything to recover.
			panic!("{}", t!("panic.extract_unstable_capsule"));
		};

		let lua: Lua = self.tour.owner.upgrade();

		return (lua, self.table.clone().unwrap());
	}

	/// Invalidates the [`Capsule`], making it unable to be used.
	/// This is best practice since it prevents [`Capsule`] from being leaked.
	/// Attempting to access the invalidated [`Capsule`] will result in a panic.
	/// To check if the [`Capsule`] is okay to use, use [`Capsule::is_stable`].
	pub(crate) fn invalidate(&mut self) -> () {
		self.tour.invalidate();
	}

	/// Returns `true` if the [`Capsule`] is okay to use, `false` otherwise.
	pub(crate) fn is_stable(&self) -> bool {
		return self.tour.stable;
	}
}

impl<T> IntoLua for Capsule<T> {
	fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
		return lua.pack(self.table.clone().unwrap());
	}
}

impl<T> FromLua for Capsule<T> {
	fn from_lua(value: Value, _: &Lua) -> ::mlua::Result<Self> {
		let table: Table = match value {
			Value::Table(table) => table,
			_ => return Err(::mlua::Error::UserDataTypeMismatch),
		};

		// This works because `Capsule` takes ownership of `Table`.
		// We capture the `Table`'s `Lua` instance here.
		return Ok(Capsule::new(table));
	}
}

impl<T> Drop for Capsule<T> {
	fn drop(&mut self) {
		self.invalidate();

		self.table = None;
	}
}
