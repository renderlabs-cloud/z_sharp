use crate::{
	lexer::consumer::Consumer,
	modification::{gluea::LuaRc, lua::LuaConstructor},
};

// ? Use `Token` as a data type?

use ::std::collections::HashMap;

use ::mlua::{Function, Table, Value};

use ::serde::{Deserialize, Serialize};

use ::mlua_magic_macros;

use ::nestify::nest;

macro_rules! impl_new {
	($type:ty) => {
		impl LuaConstructor for $type {
		}
		#[mlua_magic_macros::implementation]
		impl $type {
			fn new(value: Value) -> Result<Self, ::mlua::Error> {
				return <Self as LuaConstructor>::new(value);
			}
		}
	};
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[mlua_magic_macros::structure]
pub struct SingleDetails {
	pub pattern: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[mlua_magic_macros::structure]
pub struct RepeatDetails {
	pub rules: Vec<Rule>,
	pub seperator: Option<LuaRc<Rule>>,
	pub min: Option<usize>,
	pub max: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[mlua_magic_macros::structure]
pub struct OrDetails {
	pub rules_map: HashMap<String, Vec<Rule>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[mlua_magic_macros::structure]
pub struct ChildDetails {
	pub child: LuaRc<Chain>, // Prevent type recursion errors.
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[mlua_magic_macros::structure]
pub struct LogicDetails {
	#[serde(skip)] // Simply ignore this since serialization could be a problem here.
	pub func: Option<Function>,
}

impl_new!(SingleDetails);
impl_new!(RepeatDetails);
impl_new!(OrDetails);
impl_new!(ChildDetails);
impl_new!(LogicDetails);

mlua_magic_macros::compile!(type_path = SingleDetails, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = RepeatDetails, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = OrDetails, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = ChildDetails, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = LogicDetails, fields = true, methods = true);

nest! {
	#[derive(Clone, Serialize, Deserialize, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Chain {
		pub name: String,
		pub rules: Vec<
			#[derive(Clone, Default, Serialize, Deserialize, Debug)]
			#[mlua_magic_macros::structure]
			pub struct Rule {
				pub name: Option<String>,
				pub required: bool,
				pub details:
					#[derive(Clone, Default, Serialize, Deserialize, Debug)]
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

#[mlua_magic_macros::implementation]
impl Rule {
	pub fn new(table: Table) -> Result<Self, ::mlua::Error> {
		return Ok(Rule {
			name: table.get("name".to_string())?,
			required: table.get("required".to_string())?,
			details: table.get("details".to_string())?,
		});
	}
}

mlua_magic_macros::compile!(type_path = Rule, fields = true, methods = true);
mlua_magic_macros::compile!(type_path = RuleDetails, variants = true);

pub type CaptureResultsMap = HashMap<String, CaptureResult>;

nest! {
	#[derive(Clone, Serialize, Deserialize, Debug)]
	#[mlua_magic_macros::structure]
	pub struct CaptureResult {
		pub rule: Rule,
		pub captured: String,
		pub data: Option<
			#[derive(Clone, Default, Serialize, Deserialize, Debug)]
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
	pub fn new(name: String) -> ::mlua::Result<Self> {
		let instance: Self = Self {
			name: name,
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
