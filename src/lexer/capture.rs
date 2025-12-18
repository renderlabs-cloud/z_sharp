use crate::{
	lexer::consumer::Consumer,
	modification::gluea::{Gluea, LuaHider, LuaRc},
};

// TODO: Use `Token` as a data type?

use ::mlua::Function;

// use ::mlua::prelude::*;

use ::std::collections::HashMap;

use ::mlua_magic_macros;

use ::nestify::nest;

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct SingleDetails {
	pub pattern: String,
}

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct RepeatDetails {
	pub rules: Vec<Rule>,
	pub seperator: Option<LuaRc<Rule>>,
	pub min: Option<usize>,
	pub max: Option<usize>,
}

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct OrDetails {
	pub rules_map: HashMap<String, Vec<Rule>>,
}

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct ChildDetails {
	pub child: LuaRc<Chain>, // Prevent recursive types.
}

#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct LogicDetails {
	pub func: Option<Function>,
}

mlua_magic_macros::compile!(type_path = SingleDetails, fields = true);
mlua_magic_macros::compile!(type_path = RepeatDetails, fields = true);
mlua_magic_macros::compile!(type_path = OrDetails, fields = true);
mlua_magic_macros::compile!(type_path = ChildDetails, fields = true);
mlua_magic_macros::compile!(type_path = LogicDetails, fields = true);

nest! {
	#[derive(Clone, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Chain {
		pub(crate) glue: LuaHider<Gluea>, // Hide this with mlua-magic-macros. Add a new macro '#[mlua_magic_macros::hidden]'
		pub name: String,
		pub rules: Vec<
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
		>,
		properties: HashMap<String, String>,
	}
}

mlua_magic_macros::compile!(type_path = RuleDetails, variants = true);

mlua_magic_macros::compile!(type_path = Rule, fields = true);

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

	pub fn create_consumer(&self) -> Consumer {
		return Consumer::new(self.name.clone(), self.rules.clone());
	}

	// Captures text with Regex
	pub fn capture(&mut self, rule: Rule) -> ::mlua::Result<()> {
		self.rules.push(rule);

		return Ok(());
	}

	pub fn set_property(&mut self, key: String, value: String) -> ::mlua::Result<()> {
		self.properties.insert(key, value);

		return Ok(());
	}
}

mlua_magic_macros::compile!(type_path = Chain, fields = true, methods = true);
