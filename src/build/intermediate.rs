use crate::{
	build::{Config, error::Error, source},
	lexer::{
		capture::{CaptureResultsMap, Chain},
		consumer::Consumer,
	},
};

use ::std::collections::HashMap;

use ::mlua::ObjectLike;

pub struct Intermediate<'config> {
	pub config: &'config Config,
	pub(self) consumers: Vec<Consumer>,
	pub sources: HashMap<String, String>, // TODO: Improve the source system.
	pub source_resolvers: Vec<source::SourceResolver>,
}

// TODO: Rename?
impl<'config> Intermediate<'config> {
	/// Creates a new [`self::Intermediate`] instance.
	///
	/// This function takes a reference to a [`self::Config`] instance and creates a new [`self::Intermediate`] instance with the provided configuration.
	///
	/// The function returns a `Result` containing the created [`self::Intermediate`] instance on success, or a [`self::Error`] on failure.
	pub fn new(config: &'config Config) -> Result<Self, Error> {
		let mut intermediate: Self = Self {
			config: config,
			sources: HashMap::new(),
			source_resolvers: Vec::new(),
			consumers: Vec::new(),
		};

		for mod_ in &config.mods {
			let get_chains_result: ::mlua::Result<Vec<Chain>> = mod_
				.lua
				.globals()
				.get_path::<Vec<Chain>>("__Z_SHARP__.__UNSAFE__.registry.chains");

			match get_chains_result {
				| Ok(chains) => {
					intermediate.consumers.append(
						&mut chains
							.iter()
							.map(|chain: &Chain| {
								return chain.create_consumer();
							})
							.collect::<Vec<Consumer>>(),
					);
				},
				| Err(error) => {
					// # The __Z_SHARP__ internal proxy has failed.
					// ! This is a serious user error.

					return Err(Error::ProxyFailure(error));
				},
			};
		}

		return Ok(intermediate);
	}

	/// Registers a source resolver with the intermediate.
	///
	/// This function will register the provided source resolver with the intermediate.
	/// The source resolver will be used to resolve sources requested by the consumer.
	///
	pub fn add_source_resolvers(&mut self, source_resolver: source::SourceResolver) -> () {
		self.source_resolvers.push(source_resolver);
	}

	/// Request a source from all registered source resolvers.
	///
	/// This function will spawn a task for each source resolver, then join all the tasks.
	/// If any of the tasks succeed, the source will be added to the `sources` field of the [`self::Intermediate`] instance.
	/// If all tasks fail, an error will be returned.
	///
	/// # Errors
	///
	/// If no source resolvers are registered, an error will be returned with a message indicating that no source resolvers are available.
	///
	/// If any of the source resolvers return an error, the error will be propagated up and returned.
	///
	pub async fn request_source(&mut self, name: &'_ str) -> Result<(), self::Error> {
		let mut set: tokio::task::JoinSet<Option<String>> = tokio::task::JoinSet::new();

		for source_resolver in &self.source_resolvers {
			set.spawn(source_resolver(name.to_string()));
		}

		if set.is_empty() {
			#[allow(unreachable_code)]
			return Err(Error::NoSourceResolversAvailableFor(name.to_string()));
		};

		while let Some(join_result) = set.join_next().await {
			match join_result {
				| Ok(source_option) => {
					if let Some(source) = source_option {
						self.sources.insert(name.to_string(), source);
						return Ok(()); // Context gets destroyed since the race was successful.
					};
				},
				| Err(error) => {
					::log::error!("{}", error);
					todo!();
				},
			};
		}

		return Ok(());
	}

	/// Interpret the source associated with the given entry.
	///
	/// This function will loop through all the consumers and consume the source
	/// until no progress is made. If no progress is made, an error will be
	/// returned.
	pub async fn interpret(&mut self, entry: &'_ str) -> Result<(), Error> {
		// TODO: Blocks, ...etc.
		let source: &str = &(self.sources[entry]);

		// HACK (+) `source` is never cloned!
		let mut remaining: &str = source;

		let mut results: Vec<(String, Option<CaptureResultsMap>)> = Vec::new();

		if self.consumers.is_empty() {
			#[allow(unreachable_code)]
			return Err(Error::NoConsumers);
		};

		while !remaining.is_empty() {
			let mut was_matched: bool = false;

			'consumer_loop: for consumer in self.consumers.iter_mut() {
				let sub_results: (Option<CaptureResultsMap>, bool, &str) =
					consumer.consume(remaining);

				if sub_results.1 {
					results.push((consumer.name.clone(), sub_results.0));
					remaining = sub_results.2;

					was_matched = true;

					break 'consumer_loop;
				};
			}

			if !was_matched {
				return Err(Error::NoMatches(source::Position::default()));
			};
		}

		::log::info!("{:?}", results);

		return Ok(());
	}
}
