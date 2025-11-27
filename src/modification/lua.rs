use crate::config::VERSION;
use crate::modification::{
	// consumer::{ Consumer, },
	capture::{
		Chain, Rule,
		SingleConfig, OrConfig, RepeatConfig,
	},
	Modification,
};

use ::mlua::{
	Lua, Table, Value,
	StdLib,
	//ObjectLike,
};

use ::mlua::prelude::*;

/// Exports the Z# standard library to Lua.
pub fn z_sharp_std_lua_module(lua: & Lua, (): ()) -> mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	// TODO: Make this a struct
	exports.set("console", lua.create_table_from(vec![
		("log", lua.create_function(|_: & Lua, line: Value| {
			info!("{:#?}", line);

			return Ok(());
		})?),
		("warn", lua.create_function(|_: & Lua, line: Value| {
			warn!("{:#?}", line);

			return Ok(());
		})?),
		("error", lua.create_function(|_: & Lua, line: Value| {
			error!("{:#?}", line);

			return Ok(());
		})?),
	])?)?;

	return Ok(exports);
}

/// Exports the Z# standard library to Lua.
///
/// This function creates a new table and populates it with the following values:
///
/// - `version`: The version of the library as a table of three integers.
/// - `lexer`: A table containing functions to create new instances of the following types:
///     * `Chain`: A new `'Chain'` instance with the provided name.
///     * `Rule`: A new `'Rule'` instance.
///     * `SingleConfig`: A new `'SingleConfig'` instance.
///     * `OrConfig`: A new `'OrConfig'` instance.
///     * `RepeatConfig`: A new `'RepeatConfig'` instance.
/// - `__UNSAFE__`: A table containing an unsafe function to create a new `'Gluea'` instance.
///
/// The function returns a [`Result`] containing the created table on success, or a [`LuaError`] on failure.
pub fn z_sharp_lua_module(lua: & Lua, (): ()) -> mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	exports.set("version", [
		VERSION.major,
		VERSION.minor,
		VERSION.patch,
	])?;

	let lexer: Table = lua.create_table()?;
	lexer.set("Chain", lua.create_proxy::<Chain>()?)?;
	lexer.set("Rule", lua.create_proxy::<Rule>()?)?;
	lexer.set("SingleConfig", lua.create_proxy::<SingleConfig>()?)?;
	lexer.set("OrConfig", lua.create_proxy::<OrConfig>()?)?;
	lexer.set("RepeatConfig", lua.create_proxy::<RepeatConfig>()?)?;


	lexer.set("register_chain",
		lua.create_function(move |lua: & Lua, chain: Chain| {
			// Add a chain
			let chains: Table = lua.globals()
				.get::<Table>("__Z_SHARP__")?
				.get::<Table>("__UNSAFE__")?
				.get::<Table>("registry")?
				.get::<Table>("chains")?
			;

			let index: usize = chains.raw_len();

			chains.raw_set(index + 1, chain)?;

			return Ok(());
		})?
	)?;

	exports.set("lexer", lexer)?;

	// HACK (-) This exposes Z#'s internal registry to Lua!
	// ! Be very careful with this!
	// ? This `unsafe` block is unnecessary, but since this could lead to unexpected behavior, I will label it for now.
	#[allow(unused_unsafe)]
	unsafe {
		exports.set("__UNSAFE__", create_unsafe_portion(lua)?)?;		
	};

	return Ok(exports);
}

/// Creates an unsafe glue instance.
///
/// Use this portion carefully, as it can lead to unexpected behavior if used incorrectly.
pub fn create_unsafe_portion(lua: & Lua) -> mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	exports.set("registry",
		lua.create_table_from(vec![
			("chains".to_string(), lua.create_table()?),
		])?
	)?;

	return Ok(exports);
}

/// Creates a new 'Modification' instance.
///
/// This function takes a name and a Lua source file string. It loads the Lua source file into a new 'Lua' instance and creates a new 'Modification' instance with the provided name and the loaded Lua instance.
///
/// The function returns a 'Result' containing the created 'Modification' instance on success, or a 'LuaError' on failure.
pub fn new(name: &'static str, source: String) -> Result<Modification, LuaError> {
	let lua: Lua = Lua::new_with(
		StdLib::MATH
		| StdLib::ALL_SAFE
		,
		LuaOptions::default()
	)?;

	let globals: Table = lua.globals();

	let preload: Table = globals
		.get::<Table>("package")?
		.get::<Table>("preload")?
	;

	preload.set("z_sharp_std", lua.create_function(z_sharp_std_lua_module)?)?;
	preload.set("z_sharp", lua.create_function(z_sharp_lua_module)?)?;

	globals.set("__Z_SHARP_STD__", z_sharp_std_lua_module(&lua, ())?)?;
	globals.set("__Z_SHARP__", z_sharp_lua_module(&lua, ())?)?;

	lua.load(source).exec()?; // TODO: Handle errors?

	return Ok(Modification { 
		name: name,
		lua: lua,
	});
}