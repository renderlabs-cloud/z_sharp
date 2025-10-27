use crate::modification::capture::{Rule, };

use ::std::collections::HashMap;

use std::i32;

use ::regex::{Match, };

use ::nestify::nest;

use ::tracing::info;

nest! {
	#[derive(Default)]
	#[derive(Clone)]
	pub struct Consumer {
		pub consumed: String,
		pub rules: Vec<Rule>,
		pub index: usize,
		pub matches: Vec<
			#[derive(Default)]
			#[derive(Clone)]
			pub struct ConsumeResult {
				pub name: Option<String>,
				pub captured: String,
				pub data: Option<
				#[derive(Debug)]
				#[derive(Clone)]
				pub enum ConsumeResultData {
					Single,
					Or(HashMap<String, String>),
					Repeat(HashMap<String, Vec<String>>),
				}>,
				pub is_match: bool,
			}
		>,
		pub done: bool,
	}
}

impl Consumer {
	pub fn new(rules: Vec<Rule>) -> Consumer {
		return Consumer {
			rules: rules,
			..Default::default()
		};
	}

	pub fn check(& mut self, rule: Rule, text: String) -> ConsumeResult {
		let mut result: ConsumeResult = Default::default();

		match & rule {
			Rule::Single(config) => {
				result.name = config.name.clone();

				let capture: Option<Match<'_>> = config.pattern.find(& text);
				if let Some(mat) = capture {
					if mat.start() == 0 {
						result.data = Some(ConsumeResultData::Single);
						result.is_match = true;
						result.captured = capture.unwrap().as_str().to_string();
					};
				};
			},

			Rule::Or(config) => {
				
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

					for sub_rule in & config.rules {
						let sub_result: ConsumeResult = self.check(sub_rule.clone(), remaining_text.clone());
						
						if sub_result.is_match {
							sub_results.push(sub_result.clone());
							current_captured_text.push_str(& sub_result.captured);

							remaining_text = remaining_text[sub_result.captured.len()..].to_string();
						} else {
							all_rules_matched = false;
							break;
						};
					};

					if all_rules_matched {
						i += 1;
						total_captured.push_str(&current_captured_text);

						for result in sub_results {
							if let Some(name) = result.name {
								data_map.entry(name)
									.or_insert_with(Vec::new)
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
		};

		return result;
	}

	pub fn consume(& mut self, text: String) -> ConsumeResult {
		let rule: & Rule = & self.rules[self.index];
		let result: ConsumeResult = self.check(rule.clone(), text.clone());

		self.index += 1;
		self.consumed += & result.captured;


		info!("{:?}:{}", result.data, text);

		return result;
	}
}