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

// TODO: Refactor.
#[mlua_magic_macros::implementation]
impl RepeatConfig {
	pub fn new(table: Table) -> mlua::Result<Self> {
		return Ok(
			Self {
				name: table.get("name")?,
				rules: table.get("rules")?,
				seperator: table.get("seperator")?,
				min: table.get("min")?,
				max: table.get("max")?,
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = RepeatConfig, fields = true, methods = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct OrConfig {
	pub name: Option<String>,
	pub rules_list: HashMap<String, Vec<Rule>>,
	pub required: Option<bool>,
}

// TODO: Refactor.
#[mlua_magic_macros::implementation]
impl OrConfig {
	pub fn new(table: Table) -> mlua::Result<Self> {
		return Ok(
			Self {
				name: table.get("name")?,
				rules_list: table.get("rules_list")?,
				required: table.get("required")?,
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = OrConfig, fields = true, methods = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct SingleConfig {
	pub name: Option<String>,
	pub pattern: String,
	pub required: Option<bool>,
}

// TODO: Refactor.
#[mlua_magic_macros::implementation]
impl SingleConfig {
	pub fn new(table: Table) -> mlua::Result<Self> {
		return Ok(
			Self {
				name: table.get("name")?,
				pattern: table.get("pattern")?,
				required: table.get("required")?,
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = SingleConfig, fields = true, methods = true);

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct ChildConfig {
	pub name: Option<String>,
	pub child: LuaRc<Chain>, // Prevent recursive types.
	pub required: Option<bool>,
}

// TODO: Refactor.
#[mlua_magic_macros::implementation]
impl ChildConfig {
	pub fn new(table: Table) -> mlua::Result<Self> {
		return Ok(
			Self {
				name: table.get("name")?,
				child: table.get("child")?,
				required: table.get("required")?,
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = ChildConfig, fields = true, methods = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct LogicConfig {
	pub func: Option<Function>,
}

#[mlua_magic_macros::implementation]
impl LogicConfig {
	pub fn new(table: Table) -> mlua::Result<Self> {
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
		pub table: Table,
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
	}
}

mlua_magic_macros::compile!(type_path = Rule, variants = true);

// TODO: Major rewrite with mlua-macro-magic
#[mlua_magic_macros::implementation]
impl Chain {
	#[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
	pub fn new(name: String, gluea: Gluea) -> mlua::Result<Self> {
		let table: Table = (*gluea.borrow())
			.create_table()?
		;

		let instance: Self = Self {
			name: name,
			glue: LuaHider::new(gluea),
			rules: Vec::new(),
			table: table,
		};

		return Ok(instance);
	}

	pub fn create_consumer(& self) -> Consumer {
		return Consumer {
			rules: self.rules.clone(),
		};
	}

	// Captures text with Regex
	pub fn capture(& mut self, rule: Rule) -> mlua::Result<()> {
		self.rules.push(rule);

		return Ok(());
	}

	// Complete the Chain.
	pub fn done(& mut self) -> mlua::Result<()> {
		// info!("Rules: {:#?}", self.rules);

		// TODO: Make the chain immutable.

		return Ok(());
	}
}

mlua_magic_macros::compile!(type_path = Chain, fields = true, methods = true);