use crate::{
	modification::{
		consumer::Consumer,
		gluea::{
			Gluea,
			LuaHider,
			LuaRc,
		},
	},
};

use ::mlua::{
	Table, Function,
};

// use ::mlua::prelude::*;

use ::serde::{
	Serialize,
};

use ::std::{
	collections::HashMap,
};
use ::mlua_magic_macros;

use ::nestify::nest;


#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct RepeatConfig {
	pub name: Option<String>,
	pub rules: Vec<Rule>,
	pub seperator: Option<LuaRc<Rule>>,
	pub min: Option<usize>,
	pub max: Option<usize>,
}

mlua_magic_macros::compile!(type_path = RepeatConfig, fields = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct OrConfig {
	pub name: Option<String>,
	pub rules_list: HashMap<String, Vec<Rule>>,
	pub required: Option<bool>,
}

mlua_magic_macros::compile!(type_path = OrConfig, fields = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct SingleConfig {
	pub name: Option<String>,
	pub pattern: String,
	pub required: Option<bool>,
}

mlua_magic_macros::compile!(type_path = SingleConfig, fields = true);

#[derive(Clone, Serialize, Debug)]
#[mlua_magic_macros::structure]
pub struct ChildConfig {
	pub name: Option<String>,
	pub child: LuaRc<Chain>, // Prevent recursive types.
	pub required: Option<bool>,
}

mlua_magic_macros::compile!(type_path = ChildConfig, fields = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct LogicConfig {
	pub func: Option<Function>,
}

#[mlua_magic_macros::implementation]
impl LogicConfig {
	pub fn new(table: Table) -> ::mlua::Result<Self> {
		return Ok(
			Self {
				func: table.get("func")?,
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = LogicConfig, fields = true, methods = true);

nest! {
	#[derive(Clone, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Chain {
		pub(crate) glue: LuaHider<Gluea>, // Hide this with mlua-magic-macros. Add a new macro '#[mlua_magic_macros::hidden]'
		pub name: String,
		pub rules: Vec<
			#[derive(Clone, Default, Debug)]
			#[mlua_magic_macros::enumeration]
			pub enum Rule {
				Single(SingleConfig),
				Or(OrConfig),
				Repeat(RepeatConfig),
				Child(ChildConfig),
				Logic(LogicConfig),
				#[default]
				None,
			}
		>,
		properties: HashMap<String, String>,
	}
}

mlua_magic_macros::compile!(type_path = Rule, variants = true);

// TODO: Major rewrite with mlua-macro-magic
#[mlua_magic_macros::implementation]
impl Chain {
	#[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
	pub fn new(name: String, gluea: Gluea) -> ::mlua::Result<Self> {

		let instance: Self = Self {
			name: name,
			glue: LuaHider::new(gluea),
			rules: Vec::new(),
			properties: HashMap::new(),
		};

		return Ok(instance);
	}

	pub fn create_consumer(& self) -> Consumer {
		return Consumer {
			rules: self.rules.clone(),
		};
	}

	// Captures text with Regex
	pub fn capture(& mut self, rule: Rule) -> ::mlua::Result<()> {
		self.rules.push(rule);

		return Ok(());
	}

	// Complete the Chain.
	pub fn done(& mut self) -> ::mlua::Result<()> {
		// info!("Rules: {:#?}", self.rules);

		// TODO: Make the chain immutable.

		return Ok(());
	}

	pub fn set_property(& mut self, key: String, value: String) -> ::mlua::Result<()> {
		self.properties.insert(key, value);

		return Ok(());
	}
}

mlua_magic_macros::compile!(type_path = Chain, fields = true, methods = true);