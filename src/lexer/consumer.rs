use crate::{
	lexer::capture::{CaptureResult, CaptureResultData, CaptureResultsMap, Rule, RuleDetails},
	modification::gluea::LuaRc,
};

use ::std::collections::HashMap;

use ::regex::{Match, Regex};

#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::structure]
pub struct Consumer {
	pub name: String,
	pub rules: Vec<Rule>,
}

mlua_magic_macros::compile!(type_path = Consumer, fields = true, methods = true);

#[mlua_magic_macros::implementation]
impl Consumer {
	pub fn new(name: String, rules: Vec<Rule>) -> Consumer {
		return Consumer {
			name: name,
			rules: rules,
		};
	}
}

impl Consumer {
	#[allow(unused_labels)]
	pub fn check(
		&self,
		rule: Rule,
		(results, text): (&LuaRc<&mut CaptureResultsMap>, &str),
	) -> CaptureResult {
		let mut consumer_result: CaptureResult = {
			// HACK: (+) Don't clone the entire rule!
			let required: bool = rule.required;

			CaptureResult {
				rule: rule,
				is_match: !required,
				..Default::default()
			}
		};

		match &(consumer_result.rule).details {
			| RuleDetails::Single(config) => {
				let regex: Regex = unwrap!(Regex::new(&(config.pattern.to_string())));

				let capture: Option<Match> = regex.find(text);

				::log::info!("{}, {}, {:#?}", text, regex, capture);

				if let Some(match_) = capture
					&& match_.start() == 0
				{
					consumer_result.data = Some(CaptureResultData::Single);
					consumer_result.captured = match_.as_str().to_string(); // TODO: Optimize?
					consumer_result.is_match = true;
				};
			},

			| RuleDetails::Or(config) => {
				let mut sub_results: CaptureResultsMap = HashMap::new();
				let mut offset: usize = 0;

				'or: for sub_rules in &config.rules_map {
					'rules: for sub_rule in sub_rules.1 {
						let sub_result: CaptureResult =
							self.check(sub_rule.clone(), (results, &text[offset ..]));

						if sub_result.is_match {
							offset += sub_result.captured.len();

							sub_results.insert(sub_rules.0.clone(), sub_result);

							continue 'rules;
						} else {
							break 'rules;
						};
					}
				}

				::log::info!("Or: {:#?}: {:#?}", config.rules_map, sub_results);

				if !sub_results.is_empty() {
					consumer_result.is_match = true;
				};

				consumer_result.captured = text[0 .. offset].to_string();
			},

			| RuleDetails::Repeat(config) => {
				let min_reps: usize = config.min.unwrap_or(0);
				let max_reps: usize = config.max.unwrap_or(usize::MAX);

				let mut sub_results: Vec<CaptureResult> = Vec::new();

				let mut offset: usize = 0;
				let mut reps: usize = 0;

				if config.rules.is_empty() {
					consumer_result.is_match = true; // ? Up to debate if this should be `false` or `true`.

					// Nothing to check.
					return consumer_result;
				};

				'repeat: while reps < max_reps {
					'rules: for rule in &config.rules {
						let sub_result: CaptureResult =
							self.check(rule.clone(), (results, &text[offset ..])); // TODO: Optimize?

						if sub_result.is_match {
							sub_results.push(sub_result.clone());
							reps += 1;
							offset += sub_result.captured.len();

							continue 'rules;
						} else {
							// Break out of the while loop.
							break 'repeat;
						};
					}
				}

				if reps >= min_reps {
					consumer_result.data = Some(CaptureResultData::Repeat(sub_results));
					consumer_result.is_match = true;
					consumer_result.captured = text[0 .. offset].to_string();
				};
			},

			| RuleDetails::Child(config) => {
				let mut child_consumer: Consumer = config.child.create_consumer();
				let child_result: (Option<CaptureResultsMap>, bool, &str) =
					child_consumer.consume(text);

				match child_result.0 {
					| Some(child_result_data) => {
						// TODO: Don't use `to_string`!
						let remaining_text: &str = child_result.2;
						let captured_length: usize = text.len() - remaining_text.len();

						consumer_result.captured = text[0 .. captured_length].to_string();

						consumer_result.is_match = true;
						consumer_result.data = Some(CaptureResultData::Child(child_result_data));
					},
					| None => {
						consumer_result.is_match = true;
					},
				};
			},

			| RuleDetails::Recurse() => {
				// Since a chain cannot recurse, we need to add a new Rule type.
				let sub_results: CaptureResult = self.check(
					consumer_result.rule.clone(), /* ( 1 ) */
					(&LuaRc::new(&mut CaptureResultsMap::new()), text),
				);

				// ( 1 ): Optimize?

				consumer_result.is_match = sub_results.is_match;
				consumer_result.captured = sub_results.captured;
				consumer_result.data = sub_results.data;
			},

			| RuleDetails::Logic(config) => {
				// TODO: Add error handling.
				if let Some(func) = &config.func {
					let input: CaptureResultsMap = (***results).clone();
					let func_result: ::mlua::Result<bool> = func.call(input);

					match func_result {
						| Ok(is_match) => {
							consumer_result.is_match = is_match;
						},
						| Err(_) => {
							todo!();
						},
					};
				} else {
					todo!();
				};
			},

			| RuleDetails::Unknown => {
				::log::warn!("An unknown rule was encountered. This is probably a bug.");
				// TODO: Add bug report system.
				todo!();
			},
		};
		// ! This won't always be reached. Don't rely on it.
		// This comment is here because this block is giant.
		// TODO: Optimize.

		return consumer_result;
	}

	pub fn consume<'source>(
		&mut self,
		text: &'source str,
	) -> (Option<CaptureResultsMap>, bool, &'source str) {
		let mut remaining: &str = text;
		let mut index: usize = 0;

		let mut results: CaptureResultsMap = HashMap::new();

		while index < self.rules.len() {
			let rule: Rule = self.rules[index].clone();
			let result: CaptureResult =
				self.check(rule.clone(), (&LuaRc::new(&mut results), remaining));

			if !result.is_match {
				return (None, false, text);
			};

			index += 1;

			remaining = &(remaining[result.captured.len() ..]);

			if let Some(name) = &(result.rule.name) {
				results.insert(name.to_string(), result);
			};
		}

		return (Some(results), true, remaining);
	}
}
