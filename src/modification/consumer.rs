use crate::modification::{
	capture::{ Rule, },
	gluea::{
		LuaRc,
	},
};

use ::std::collections::HashMap;

use ::regex::{
	Regex,
	Match,
};

use ::nestify::nest;


#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct Consumer {
	pub rules: Vec<Rule>,
}

nest! {
	#[derive(Clone, Default, Debug)]
	#[mlua_magic_macros::structure]
	pub struct CaptureResult {
		pub name: Option<String>,
		pub captured: String,
		pub data: Option<
		#[derive(Clone, Default, Debug)]
		#[mlua_magic_macros::enumeration]
		pub enum CaptureResultData {
			Single,
			Or(Results),
			Repeat(Vec<CaptureResult>),
			#[default]
			None,
		}>,
		pub is_match: bool,
	}
}

pub type Results = HashMap<String, CaptureResult>;

mlua_magic_macros::compile!(type_path = CaptureResult, fields = true);

mlua_magic_macros::compile!(type_path = CaptureResultData, variants = true);

mlua_magic_macros::compile!(type_path = Consumer, fields = true, methods = true);

#[mlua_magic_macros::implementation]
impl Consumer {
	pub fn new(rules: Vec<Rule>) -> Consumer {
		return Consumer {
			rules: rules,
		};
	}
}

impl Consumer {
	#[allow(unused_labels)]
	pub fn check(& self, rule: Rule, (results, text): (& LuaRc<&mut Results>, & str)) -> CaptureResult {
		let mut result: CaptureResult = Default::default();

		match & rule {
			Rule::Single(config) => {
				result.name = config.name.clone();

				let regex: Regex = Regex::new(
					&(config.pattern.clone().to_string())
				).unwrap();

				let capture: Option<Match<'_>> = regex.find(text);

				if let Some(match_) = capture && match_.start() == 0 {
					result.data = Some(CaptureResultData::Single);
					result.is_match = true;
					result.captured = match_.as_str().to_string();
				};
			},

			Rule::Or(config) => {
				let mut sub_results: HashMap<String, CaptureResult> = HashMap::new();
				let mut offset: usize = 0;

				info!("Config: {:#?}", config);

				'or: for sub_rules in &config.rules_list {
					'rules: for sub_rule in sub_rules.1 {
						let sub_result: CaptureResult = self.check(sub_rule.clone(), (results, text));
						
						info!("Sub result: {:#?}", sub_result);
						if sub_result.is_match {
							offset += sub_result.captured.len();

							sub_results.insert(sub_rules.0.clone(), sub_result);

							break 'or;
						};
					};

				};

				if !sub_results.is_empty() {
					result.is_match = true;
				};

				if let Some(required) = config.required && required {
					result.is_match = true;
				};

				result.data = Some(CaptureResultData::Or(sub_results));
				result.name = config.name.clone();
				result.captured = text[0 .. offset].to_string();
			},

			Rule::Repeat(config) => {
				let min_reps: usize = config.min.unwrap_or(1);
				let max_reps: usize = config.max.unwrap_or(usize::MAX);

				let mut sub_results: Vec<CaptureResult> = Vec::new();

				let mut offset: usize = 0;
				let mut reps: usize = 0;

				if config.rules.is_empty() {
					result.is_match = true; 
					
				};

				'repeat: while reps < max_reps {
					'rules: for rule in &config.rules {
						let sub_result: CaptureResult = self.check(rule.clone(), (results, &text[offset..])); // TODO: Optimize?

						if sub_result.is_match {
							sub_results.push(sub_result.clone());
							reps += 1;
							offset += sub_result.captured.len();

							continue 'rules;
						} else {
							// Break out of the while loop.
							break 'repeat;
						};
					};
				};

				if reps >= min_reps {
					result.data = Some(CaptureResultData::Repeat(sub_results));
					result.is_match = true;
					result.captured = text[0 .. offset].to_string();
				} else {
					result.is_match = false;
				};
			},

			Rule::Logic(config) => {
				// TODO: Add error handling.
				if let Some(func) = &config.func {
					let func_result: mlua::Result<bool> = func.call((***results).clone());

					match func_result {
						Ok(is_match) => {
							result.is_match = is_match;
						},
						Err(_) => {
							todo!();
						},
					};
				} else {
					todo!();
				};
			},

			Rule::None => {
				result.is_match = true;
			},
		};

		return result;
	}

	pub fn consume<'a>(& mut self, text: &'a str) -> (Option<Results>, &'a str) {
		let mut remaining: & str = text;
		let mut index: usize = 0;

		let mut results: Results = HashMap::new();

		while index < self.rules.len() {
			let rule: Rule = self.rules[index].clone();
			let result: CaptureResult = self.check(rule.clone(), (&LuaRc::new(&mut results), remaining));

			if !result.is_match {
				info!("{}", text);
				return (None, text);
			};

			index += 1;

			remaining = &(remaining[result.captured.len() ..]);

			if let Some(name) = &(result.name) {
				results.insert(name.to_string(), result);
			};
			
		};

		return (Some(results), remaining);
	}
}