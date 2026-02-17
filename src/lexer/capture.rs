use crate::{
	lexer::consumer::Consumer,
	modification::{
		gluea::LuaRc,
		lua::{
			self,
			capsule::{self, Capsule},
			magic::{lua_table},
		},
	},
};

use ::std::collections::HashMap;

#[rustfmt::skip]
use ::mlua::{
	Lua,
	Value, Function, Table,
	IntoLua, FromLua,
	UserData,
};

use ::mlua_magic_macros;

use ::nestify::nest;

lua_table!(SingleDetails { pattern: String });

lua_table!(
	RepeatDetails {
		rules: Vec<Rule>,
		seperator: Option<LuaRc<Rule>>,
		min: Option<usize>,
		max: Option<usize>,
	}
);

lua_table!(
	OrDetails {
		rules_map: HashMap<String, Vec<Rule>>,
	}
);

lua_table!(
	ChildDetails {
		child: LuaRc<Chain>,
	}
);

lua_table!(
	LogicDetails {
		func: Option<LuaRc<Function>>,
	}
);

nest! {
	#[derive(Clone, Default, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Rule {
		pub name: Option<String>,
		pub required: bool,
		pub details:
			#[derive(Clone, Default, Debug)]
			#[mlua_magic_macros::enumeration]
			pub enum RuleDetails {
				Single(SingleDetails),
				Or(OrDetails),
				Repeat(RepeatDetails),
				Child(ChildDetails),
				Recurse(),
				Logic(LogicDetails),
				#[default]
				Unknown, // ? Usually this is a bad sign.
			}
		,
	}
}

#[mlua_magic_macros::implementation]
impl Rule {
	pub fn new(capsule: Capsule<Self>) -> Result<Self, ::mlua::Error> {
		if !capsule.is_stable() {
			return Err(::mlua::Error::UserDataTypeMismatch);
		};

		let table: Table = capsule.extract().1;

		// Extract the metadata.
		let name: Option<String> = table.get("name")?;
		let required: bool = table.get("required").unwrap_or(false);

		// Determine which enum variant to use based on the "type" field in Lua.
		let type_str: String = table.get("type")?;

		let details: Table = table.get("details")?;

		let details: RuleDetails = match type_str.as_str() {
			"single" => RuleDetails::Single(SingleDetails::new(details)?),
			"or" => RuleDetails::Or(OrDetails::new(details)?),
			"repeat" => RuleDetails::Repeat(RepeatDetails::new(details)?),
			"child" => RuleDetails::Child(ChildDetails::new(details)?),
			"recurse" => RuleDetails::Recurse(),
			"logic" => RuleDetails::Logic(LogicDetails::new(details)?),
			_ => RuleDetails::Unknown,
		};

		return Ok(Self {
			name: name,
			required: required,
			details: details,
		});
	}
}

mlua_magic_macros::compile!(type_path = Rule, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = RuleDetails, variants = true);

pub type CaptureResultsMap = HashMap<String, CaptureResult>;

nest! {
	#[derive(Clone, Debug)]
	#[mlua_magic_macros::structure]
	pub struct CaptureResult {
		pub rule: Rule,
		pub captured: String,
		pub data: Option<
			#[derive(Clone, Default, Debug)]
			#[mlua_magic_macros::enumeration]
			pub enum CaptureResultData {
				Single,
				Or(CaptureResultsMap),
				Repeat(Vec<CaptureResult>),
				Child(CaptureResultsMap),
				Recurse(CaptureResultsMap),
				#[default]
				Unknown,
			},
		>,
		pub is_match: bool,
	}
}

mlua_magic_macros::compile!(type_path = CaptureResult, fields = true);
mlua_magic_macros::compile!(type_path = CaptureResultData, variants = true);

impl Default for CaptureResult {
	fn default() -> Self {
		return Self {
			rule: Rule::default(),
			captured: String::new(),
			data: None,
			is_match: false,
		};
	}
}

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct Chain {
	pub name: String,
	pub rules: Vec<Rule>,
	pub properties: HashMap<String, String>,
}

#[mlua_magic_macros::implementation]
impl Chain {
	pub fn new(name: String) -> ::mlua::Result<Self> {
		let instance: Self = Self {
			name: name,
			rules: Vec::new(),
			properties: HashMap::new(),
		};

		return Ok(instance);
	}

	/// Generate a consumer from the chain.
	pub fn create_consumer(&self) -> Consumer {
		return Consumer::new(self.name.clone(), self.rules.clone());
	}

	/// Adds a rule to the chain.
	pub fn capture(&mut self, rule: Rule) -> ::mlua::Result<()> {
		self.rules.push(rule);

		return Ok(());
	}

	/// Adds multiple rules to the chain.
	pub fn capture_all(&mut self, rules: Vec<Rule>) -> ::mlua::Result<()> {
		self.rules.extend(rules);

		return Ok(());
	}

	// TODO: Implement or remove.
	pub fn set_property(&mut self, key: String, value: String) -> ::mlua::Result<()> {
		self.properties.insert(key, value);

		return Ok(());
	}
}

mlua_magic_macros::compile!(type_path = Chain, fields = true, methods = true);
