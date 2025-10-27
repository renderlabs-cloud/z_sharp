use crate::version::VERSION;
use crate::modification::consumer::{Consumer, };
use crate::modification::Modification;
use crate::modification::capture::Chain;

use ::mlua::{Lua, Table, Function, Value};

use ::mlua::prelude::*;
use ::mlua::Error;
use ::mlua::StdLib;

use ::tracing::{info, };

use ::std::collections::HashMap;

pub fn load_std(lua: & Lua) -> Result<(), Error> {
	lua.globals().set("console", lua.create_table_from(vec![
		("log", lua.create_function(|_: & Lua, line: Value| {
			info!("{:#?}", line);

			return Ok(());
		})?),
	])?)?;

	return Ok(());
}

pub fn load_z_sharp_module(lua: & Lua) -> Result<(), Error> {
	load_std(& lua)?;

	let package: Table = lua.globals().get("package")?;
	let searchers: Table = package.get("searchers")?;

	let searcher: Function = lua.create_function(move |lua: & Lua, name: String| {
		let loader: Function = lua.create_function(move |lua: & Lua, ()| {
			let exports: Table = lua.create_table()?;
			exports.set("version", [
				VERSION.major,
				VERSION.minor,
				VERSION.patch,
			])?;

			exports.set("__", lua.create_table_from([
				("capture_zones", lua.create_table()?),
			])?)?;

			exports.set("create_capture_zone", lua.create_function(move |lua: & Lua, name: String| {
				// ? Lua is currently in the z_sharp module
				let chain: Chain = Chain::new(&lua)?;

				// TODO: Save `name` to capture_zones
				info!(name);
				let internals: Table = lua.globals();
				internals.push(chain)?;
				info!("{:#?}", secret);

				return Ok(chain.table.clone());
			})?)?;

			return Ok(exports.clone());
		})?;

		if name == "z_sharp" {
			return Ok(mlua::Value::Function(loader));
		} else {
			return Ok(mlua::Value::Nil);
		};
	})?;

	searchers.raw_insert(1, searcher)?;

	return Ok(());
}

pub fn new(name: &'static str, source: String) -> Result<Modification, Error> {
	let lua: Lua = Lua::new_with(
		StdLib::MATH
		| StdLib::COROUTINE
		| StdLib::STRING
		| StdLib::PACKAGE
		| StdLib::TABLE
		,
		LuaOptions::new()
	)?;

	let exports: HashMap<String, Value> = HashMap::new();

	load_z_sharp_module(&lua)?;

	lua.load(source).exec()?;

	return Ok(Modification { 
		name: name,
		ctx: lua,
		exports: exports
	});
}