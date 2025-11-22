use crate::modification::{
	capture::{ Rule, },
};

use ::std::collections::HashMap;

use ::regex::{ Regex, Match, };

use ::nestify::nest;

nest! {
	#[derive(Clone, Default, Debug)]
	#[mlua_magic_macros::structure]
	pub struct Consumer {
		pub consumed: String,
		pub rules: Vec<Rule>,
		pub index: usize,
		pub results: HashMap<String,
			#[derive(Clone, Default, Debug)]
			#[mlua_magic_macros::structure]
			pub struct ConsumeResult {
				pub name: Option<String>,
				pub captured: String,
				pub data: Option<
				#[derive(Clone, Default, Debug)]
				#[mlua_magic_macros::enumeration]
				pub enum ConsumeResultData {
					Single,
					Or(HashMap<String, String>),
					Repeat(HashMap<String, Vec<String>>),
					#[default]
					None,
				}>,
				pub is_match: bool,
			}
		>,
		pub done: bool,
		pub is_match: bool,
	}
}

mlua_magic_macros::compile!(type_path = ConsumeResultData, variants = true);

mlua_magic_macros::compile!(type_path = ConsumeResult, fields = true);

mlua_magic_macros::compile!(type_path = Consumer, fields = true, methods = true);

#[mlua_magic_macros::implementation]
impl Consumer {
	pub fn new(rules: Vec<Rule>) -> Consumer {
		return Consumer {
			rules: rules,
			..Default::default()
		};
	}

	#[allow(clippy::only_used_in_recursion)]
	pub fn check(& self, rule: Rule, text: String) -> ConsumeResult {
		let mut result: ConsumeResult = Default::default();

		match & rule {
			Rule::Single(config) => {
				result.name = config.name.clone();

				let regex: Regex = Regex::new(
					&(config.pattern.clone().to_string())
				).unwrap();

				let capture: Option<Match<'_>> = regex.find(&text);

				if let Some(mat) = capture && mat.start() == 0 {
					result.data = Some(ConsumeResultData::Single);
					result.is_match = true;
					result.captured = capture.unwrap().as_str().to_string();
				};
			},

			Rule::Or(_config) => {
				// TODO:
			},

			Rule::Repeat(config) => {
				result.name = config.name.clone();

				let mut data_map: HashMap<String, Vec<String>> = HashMap::new();
				let mut remaining_text: String = text.clone();
				let mut total_captured: String = String::new();

				let min_reps: i32 = config.min.unwrap_or(0);
				let max_reps: i32 = config.max.unwrap_or(i32::MAX);

				let mut i: i32 = 0;

				while i < max_reps {
					let mut all_rules_matched: bool = true;
					let mut sub_results: Vec<ConsumeResult> = Vec::new();
					
					let mut current_captured_text: String = String::new();

					for sub_rule in &config.rules {
						let sub_result: ConsumeResult = self.check(sub_rule.clone(), remaining_text.clone());
						
						if sub_result.is_match {
							sub_results.push(sub_result.clone());
							current_captured_text.push_str(&(sub_result.captured));

							remaining_text = remaining_text[sub_result.captured.len()..].to_string();
						} else {
							all_rules_matched = false;
							break;
						};
					};

					if all_rules_matched {
						i += 1;
						total_captured.push_str(& current_captured_text);

						for result in sub_results {
							if let Some(name) = result.name {
								data_map.entry(name.to_string())
									.or_default()
									.push(result.captured)
								;
							};
						};
					} else {
						break;
					};
				};

				if i >= min_reps {
					result.is_match = true;
					result.captured = total_captured;
					result.data = Some(ConsumeResultData::Repeat(data_map));
				};
			},

			Rule::Logic(config) => {
				// TODO: Add error handling.
				if let Some(func) = &config.func {
					let func_result: mlua::Result<bool> = func.call(self.results.clone());

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
}

impl Consumer {
	pub fn consume<'a>(& mut self, text: &'a str) -> &'a str {
		let mut remaining: & str = text;

		self.is_match = true;

		while self.index < self.rules.len() {
			let rule: Rule = self.rules[self.index].clone();
			let result: ConsumeResult = self.check(rule.clone(), remaining.to_string());

			if !result.is_match {
				self.is_match = false;
				info!("{}", text);
				return text;
			};

			self.index += 1;
			self.consumed += &(result.captured);

			remaining = &(remaining[result.captured.len() ..]);

			if let Some(name) = &result.name {
				self.results.insert(name.to_string(), result);
			};
			
		};

		self.done = true;

		return remaining;
	}
}