use crate::modification::{
	consumer::Consumer,
	gluea::{
		Gluea,
		LuaBox,
	},
};

use ::mlua::{
	Lua, Table, Value, Function,
	IntoLua,
};
// use ::mlua::prelude::*;

use ::mlua_magic_macros;

use ::nestify::nest;


#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct RepeatConfig {
	pub name: Option<String>,
	pub rules: Vec<Rule>,
	pub seperator: Option<LuaBox<Rule>>,
	pub min: Option<i32>,
	pub max: Option<i32>,
}

mlua_magic_macros::compile!(type_path = RepeatConfig, fields = true);

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct OrConfig {
	pub name: Option<String>,
	pub rules: Vec<Rule>,
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

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct LogicConfig {
	pub func: Option<Function>,
}

#[mlua_magic_macros::implementation]
impl LogicConfig {
	pub fn new(func: Function) -> mlua::Result<Self> {
		return Ok(
			Self {
				func: Some(func),
			}
		);
	}
}

mlua_magic_macros::compile!(type_path = LogicConfig, fields = true, methods = true);

nest! {
	#[derive(Clone, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Chain {
		pub(crate) glue: Gluea, // Hide this with mlua-magic-macros. Add a new macro `#[mlua_magic_macros::hidden]`
		pub name: String,
		pub table: Table,
		pub rules: Vec<
			#[derive(Clone, Default, Debug)]
			#[mlua_magic_macros::enumeration]
			pub enum Rule {
				Single(SingleConfig),
				Or(OrConfig),
				Repeat(RepeatConfig),
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
	/// This method doesn't make a `Chain`!
	/// It's an internal method for chaining the calls.
	pub(self) fn chain(& self) -> mlua::Result<LuaBox<Value>> {
		return Ok(
			LuaBox(
				Box::new(
					self
						.clone()
						.into_lua(&(self.glue.borrow()))?
				)
			)
		);
	}

	pub fn create_consumer(& self) -> Consumer {
		return Consumer {
			rules: self.rules.clone(),
			index: 0,
			..Default::default()
		};
	}

	// Captures text with Regex
	pub fn capture(& mut self, rule: Rule) -> mlua::Result<LuaBox<Value>> {
		self.rules.push(rule);

		return self.chain();
	}

	// Removes whitespace.
	pub fn trim(& mut self) -> mlua::Result<LuaBox<Value>> {
		// TODO: Make this configurable.
		let trimmer: Rule = Rule::Single(SingleConfig {
			name: None,
			pattern: "\\s*".to_string(),
			required: Some(false), // Actually, false doesn't mean much here, but just in case let's keep it here until we make it configurable.
		});

		self.rules.push(trimmer);

		return self.chain();
	}

	// Define logic for capture group.
	// TODO: Improve logic capabilities.
	pub fn logic(& mut self, func: Function) -> mlua::Result<LuaBox<Value>> {
		let rule: Rule = Rule::Logic(LogicConfig::new(func).unwrap()); // TODO: Error handling?

		self.rules.push(rule);

		return self.chain();
	}

	// Complete the Chain.
	pub fn done(& mut self) -> mlua::Result<()> {
		info!("Registering \"{}\".", self.name);
		let lua: & Lua = &(self.glue).borrow();
		let exporter: Function = lua.load("require('z_sharp').__UNSAFE__.export_chain").eval()?; // TODO: Expand.

		exporter.call::<Chain>((*self).clone())?;

		return Ok(());
		
	}
}

mlua_magic_macros::compile!(type_path = Chain, fields = true, methods = true);

impl Chain {
	pub fn new(name: String, lua: & Lua) -> Self {
		let instance: Self = Self {
			name: name,
			glue: Gluea::new(lua.clone()), // TODO: Optimize?
			rules: Vec::new(),
			table: lua.create_table().unwrap(), // ? This should never fail.
		};

		return instance;
	}
}