use crate::config::VERSION;
// use crate::modification::consumer::{ };
use crate::modification::{ Modification, };
use crate::modification::capture::{ Chain, Rule, SingleConfig, OrConfig, RepeatConfig, };

use ::mlua::{
	Lua, Table, Value,
	StdLib,
	ObjectLike,
};

use ::mlua::prelude::*;

pub fn z_sharp_std_lua_module(lua: & Lua, (): ()) -> mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	// TODO: Make this a struct
	exports.set("console", lua.create_table_from(vec![
		("log", lua.create_function(|_: & Lua, line: Value| {
			info!("{:#?}", line);

			return Ok(());
		})?),
	])?)?;

	return Ok(exports);
}

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
	
	lexer.set("create_chain", 
		lua.create_function(move |lua: & Lua, name: String| {
			let chain: Chain = Chain::new(name, lua);

			chain.table.set("name", chain.name.clone())?;

			return Ok(chain.clone());
		})?
	)?;

	exports.set("lexer", lexer)?;

	exports.set("__UNSAFE__", create_unsafe_portion(lua)?)?;

	return Ok(exports.clone());
}

pub fn create_unsafe_portion(lua: & Lua) -> mlua::Result<Table> {
	let exports: Table = lua.create_table()?;

	exports.set("__EXPORTS__",
		lua.create_table_from(vec![
			("chains".to_string(), lua.create_table()?),
		])?
	)?;

	exports.set("export_chain",
		lua.create_function(move |lua: & Lua, chain: Chain| {
			// Add a chain
			let chains: Table = lua.globals().get_path::<Table>("__Z_SHARP__.__UNSAFE__.__EXPORTS__.chains")?;

			let index: usize = chains.raw_len();

			chains.raw_set(index + 1, chain)?;

			return Ok(());
		})?
	)?;

	return Ok(exports);
}

pub fn new(name: &'static str, source: String) -> Result<Modification, LuaError> {
	let lua: Lua = Lua::new_with(
		StdLib::MATH
		// | StdLib::COROUTINE To be considered
		| StdLib::STRING
		| StdLib::PACKAGE
		| StdLib::TABLE
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