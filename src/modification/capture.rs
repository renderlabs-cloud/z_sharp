use crate::modification::consumer::{Consumer };

use ::mlua::{Lua, Table, Value};
use ::mlua::prelude::*;

use ::serde::{Deserialize, };

use ::std::sync::{Arc, Mutex};

use ::regex::Regex;

use ::nestify::nest;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct RepeatConfig {
	pub name: Option<String>,
	pub rules: Vec<Rule>,
	pub seperator: Option<Box<Rule>>,
	pub min: Option<i32>,
	pub max: Option<i32>,
}

#[derive(Clone)]
#[derive(Deserialize)]
pub struct OrConfig {
	pub name: Option<String>,
	pub rules: Vec<Rule>,
	pub required: Option<bool>,
}

#[derive(Clone)]
#[derive(Deserialize)]
pub struct SingleConfig {
	pub name: Option<String>,
	#[serde(with = "serde_regex")]
	pub pattern: Regex,
	pub required: Option<bool>,
}

nest! {
	#[derive(Clone)]
	#[derive(Deserialize)]
	pub struct Chain {
		pub table: Value,
		pub rules: Arc<Mutex<Vec<
			#[derive(Clone)]
			#[derive(Deserialize)]
			pub enum Rule {
				Single(SingleConfig),
				Or(OrConfig),
				Repeat(RepeatConfig),
			},
		>>>,
	}
}

impl Chain {
	pub fn new(lua: & Lua) -> Result<Chain, LuaError> {
		let chain: Chain = Chain { 
			table: lua.create_table()?.to_value(),
			
			rules: Arc::new(Mutex::new(Vec::new())),
		};

		let rules_clone: Arc<Mutex<Vec<Rule>>> = Arc::clone(&chain.rules);
		let table_clone: Table = chain.table.into()?;

		chain.table.as_table().unwrap().set("capture", lua.create_function(move |lua: & Lua, config_table: Value| {
			let config: Rule = lua.from_value(config_table)?;

			rules_clone.lock().unwrap().push(config.clone());

			return Ok(table_clone.clone());
		})?)?;

		let rules_clone: Arc<Mutex<Vec<Rule>>> = Arc::clone(&chain.rules);
		let table_clone: Table = chain.table.clone();

		chain.table.set("trim", lua.create_function(move |_: &Lua, _: Value| {
			let trimmer: Rule = Rule::Single(SingleConfig {
				pattern: Regex::new("\\s*").unwrap(),
				name: None,
				required: Some(false),
			});

			rules_clone.lock().unwrap().push(trimmer.clone());

			return Ok(table_clone.clone());
		})?)?;

		let chain_clone: Chain = chain.clone();

		chain.table.set("done", lua.create_function(move |_: & Lua, _: Value| {
			

			return Ok(chain_clone.table.clone());
		})?)?;

		return Ok(chain);
	}
	pub fn create_consumer(& self) -> Consumer {
		return Consumer {
			rules: self.rules.clone().lock().unwrap().to_vec(),
			index: 0,
			..Default::default()
		};
	}
}