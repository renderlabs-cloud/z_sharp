use crate::config::VERSION;
use crate::{
	lexer::{
		// consumer::{ Consumer, },
		capture::{
			Chain,
			ChildDetails,
			LogicDetails,
			OrDetails,
			RepeatDetails,
			Rule,
			RuleDetails,
			SingleDetails,
		},
	},
	modification::Modification,
};

use ::mlua::{Lua, StdLib, Table, Value};

use ::mlua::prelude::*;

/// Exports the Z# internal library to Lua.
///
/// The function returns a [`Result`] containing the created table on success, or a [`LuaError`] on failure.
pub fn load_z_sharp_lua_module(lua: &Lua, (): ()) -> ::mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	exports.set("version", [VERSION.major, VERSION.minor, VERSION.patch])?;

	let lexer: Table = lua.create_table()?;

	lexer.set("Chain", lua.create_proxy::<Chain>()?)?;
	lexer.set("Rule", lua.create_proxy::<Rule>()?)?;
	lexer.set("RuleDetails", lua.create_proxy::<RuleDetails>()?)?;
	lexer.set("RuleDetails", lua.create_proxy::<RuleDetails>()?)?;
	lexer.set("SingleDetails", lua.create_proxy::<SingleDetails>()?)?;
	lexer.set("OrDetails", lua.create_proxy::<OrDetails>()?)?;
	lexer.set("RepeatDetails", lua.create_proxy::<RepeatDetails>()?)?;
	lexer.set("ChildDetails", lua.create_proxy::<ChildDetails>()?)?;
	lexer.set("LogicDetails", lua.create_proxy::<LogicDetails>()?)?;

	lexer.set(
		"register_chain",
		lua.create_function(move |lua: &Lua, chain: Chain| {
			// Add a chain
			let chains: Table = lua
				.globals()
				.get::<Table>("__Z_SHARP__")? // For internal use.
				.get::<Table>("__UNSAFE__")?
				.get::<Table>("registry")?
				.get::<Table>("chains")?;

			let index: usize = chains.raw_len();

			chains.raw_set(index + 1, chain)?;

			return Ok(());
		})?,
	)?;

	exports.set("lexer", lexer)?;

	let console: Table = lua.create_table_from(vec![
		(
			"log",
			lua.create_function(|_: &Lua, line: Value| -> ::mlua::Result<()> {
				::log::info!("{:#?}", line);

				return Ok(());
			})?,
		),
		(
			"warn",
			lua.create_function(|_: &Lua, line: Value| -> ::mlua::Result<()> {
				::log::warn!("{:#?}", line);

				return Ok(());
			})?,
		),
		(
			"error",
			lua.create_function(|_: &Lua, line: Value| -> ::mlua::Result<()> {
				::log::error!("{:#?}", line);

				return Ok(());
			})?,
		),
	])?;

	exports.set("console", console)?;

	// HACK (+/-) This exposes Z#'s internal registry to Lua!
	// /!\ Ensure there are no security vulnerabilities!
	exports.set("__UNSAFE__", create_unsafe_portion(lua)?)?;

	return Ok(exports);
}

/// Creates an unsafe glue instance.
///
/// Use this portion carefully, as it can lead to unexpected behavior if used incorrectly.
pub fn create_unsafe_portion(lua: &Lua) -> ::mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	exports.set(
		"registry",
		lua.create_table_from(vec![
			("whitespace".to_string(), lua.null()),
			("chains".to_string(), lua.create_table()?.to_value()),
		])?,
	)?;

	return Ok(exports);
}

/// Creates a new [`Modification`] instance.
///
/// This function takes a name and a Lua source file string. It loads the Lua source file into a new [`Lua`] instance and creates a new [`Modification`] instance with the provided name and the loaded Lua instance.
///
/// The function returns a [`Result`] containing the created [`Modification`] instance on success, or a [`LuaError`] on failure.
pub fn new(name: &'static str, source: String) -> Result<Modification, LuaError> {
	let lua: Lua = Lua::new_with(
		StdLib::BIT
			| StdLib::BUFFER
			| StdLib::COROUTINE
			| StdLib::DEBUG
			| StdLib::MATH
			| StdLib::STRING
			| StdLib::TABLE
			| StdLib::UTF8
			| StdLib::VECTOR,
		LuaOptions::default(),
	)?;

	let z_sharp_module: &Table = &load_z_sharp_lua_module(&lua, ())?;
	lua.register_module("@z_sharp", z_sharp_module)?;

	let globals: Table = lua.globals();
	globals.set("__Z_SHARP__", z_sharp_module)?;

	lua.sandbox(true)?;
	lua.load(source).exec()?;

	return Ok(Modification {
		name: name,
		lua: lua,
	});
}
