pub mod builder {
	use crate::modification::{
		Modification,
		capture::{
			Chain,
			Rule,
			SingleConfig,
		},
		consumer::{
			Consumer,
			ResultsMap as ConsumerResultsMap,
		},
		// gluea::{ Gluea, },
	};

	use ::std::collections::{ HashMap, };
	use ::std::future::{ Future, };
	use ::std::pin::{ Pin, };
	use ::std::time::{
		Instant,
	};

	use ::anyhow::{
		Result,
	};

	use ::mlua::{
		ObjectLike,
	};

	use ::tokio;

	pub type SourceResolverKernel = dyn Future<Output = Option<String>> + Send;

	pub type SourceResolver = Box<
		dyn Fn(String) -> Pin<Box<self::SourceResolverKernel>> + Send + Sync,
	>;

	pub struct Config {
		pub mods: Vec<Modification>,
		// ...

	}

	pub struct Intermediate<'a> {
		pub config: &'a self::Config,
		pub(self) consumers: Vec<Consumer>,
		pub sources: HashMap<String, String>, // TODO: Improve the source system.
		pub source_resolvers: Vec<SourceResolver>,
	}

	#[derive(Debug)]
	pub struct Position {
		pub line: u32,
		pub column: u32,
	}

	#[derive(Debug)]
	pub enum Error {
		NoConsumers,
		NoMatches(Position),
		NoSourceResolversAvailable,
		NoSourceResolversAvailableFor(String),
		LuauError,
	}

	impl<'a> Intermediate<'a> {

		/// Creates a new `Intermediate` instance.
		///
		/// This function takes a reference to a `Config` instance and creates a new `Intermediate` instance with the provided configuration.
		///
		/// The function returns a `Result` containing the created `Intermediate` instance on success, or a `Error` on failure.
		pub fn new(config: &'a self::Config) -> Result<Self, self::Error> {
			let mut intermediate: Self = Self {
				config: config,
				sources: HashMap::new(),
				source_resolvers: Vec::new(),
    			consumers: Vec::new(),
			};

			intermediate.consumers.push(
				Consumer { 
					rules: vec![
						Rule::Single(SingleConfig {
							name: None,
							pattern: "\\s+".to_string(),
							required: Some(false),
						})
					],
				}
			);
			
			return Ok(intermediate);
		}

		/// Registers a source resolver with the intermediate.
		///
		/// This function will register the provided source resolver with the intermediate.
		/// The source resolver will be used to resolve sources requested by the consumer.
		///
		pub fn add_source_resolvers(& mut self, source_resolver: self::SourceResolver) -> () {
			self.source_resolvers.push(source_resolver);
		}

		/// Request a source from all registered source resolvers.
		///
		/// This function will spawn a task for each source resolver, then join all the tasks.
		/// If any of the tasks succeed, the source will be added to the 'sources' field of the 'Intermediate' instance.
		/// If all tasks fail, an error will be returned.
		///
		/// # Errors
		///
		/// If no source resolvers are registered, an error will be returned with a message indicating that no source resolvers are available.
		///
		/// If any of the source resolvers return an error, the error will be propagated up and returned.
		/// 
		pub async fn request_source(& mut self, name: & str) -> Result<(), self::Error> {
			let mut set: tokio::task::JoinSet::<Option<String>> = tokio::task::JoinSet::new();

			for source_resolver in 	& self.source_resolvers {
				set.spawn(source_resolver(name.to_string()));
			};
			
			if set.is_empty() {
				#[allow(unreachable_code)]
				return Err(self::Error::NoSourceResolversAvailableFor(name.to_string()));
			};

			while let Some(join_result) = set.join_next().await {
				match join_result {
					Ok(source_option) => {
						if let Some(source) = source_option {
							self.sources.insert(name.to_string(), source);
							return Ok(()); // Context gets destroyed since the race was successful.
						};
 					},
					Err(err) => {
						::log::error!("{}", err);
						todo!();
					},
				};
			};

			return Ok(());
		}

		/// Interpret the source associated with the given entry.
		///
		/// This function will loop through all the consumers and consume the source
		/// until no progress is made. If no progress is made, an error will be
		/// returned.
		pub async fn interpret(&mut self, entry: & str) -> Result<(), self::Error> {
			// TODO: Blocks, ...etc.
			let start_time: Instant = Instant::now();

			let source: & str = &(self.sources[entry]);

			// HACK (+) `source` is never cloned!
			let mut remaining: & str = source;

			if self.consumers.is_empty() {
				#[allow(unreachable_code)]
				return Err(self::Error::NoConsumers);
			};

			'capture_loop:
			while !remaining.is_empty() {
				let results: Vec<(Option<ConsumerResultsMap>, bool, & str)>= self.consumers
					.iter_mut()
					.map(|consumer: &mut Consumer| -> (Option<ConsumerResultsMap>, bool, & str) {
						return consumer.consume(remaining);
					})
					.collect::<Vec<_>>()
				;
				let mut was_matched: bool = false;

				'consumer_loop: 
				for output in results.iter() {
					if !output.1 {
						continue 'consumer_loop;
					};

					remaining = output.2;
					was_matched = true;

					if remaining.is_empty() {
						break 'capture_loop;
					};
				};

				if !was_matched {
					#[allow(unreachable_code)]
					return Err(self::Error::NoMatches(self::Position {
						line: 0,
						column: 0,
					})); // TODO: Implement position.
				};

				::log::info!("Characters remaining: {}", remaining.len());
			};

			let end_time: Instant = Instant::now();
			::log::info!("Interpretation took {}ms.", end_time.duration_since(start_time).as_millis());

			return Ok(());
		}
	}

	/// Creates a new 'Intermediate' instance.
	///
	/// This function takes a 'Config' instance and uses it to create a new 'Intermediate' instance.
	///
	/// The function returns a 'Result' containing the created 'Intermediate' instance on success, or an 'Error' on failure.
	pub fn new<'a>(config: &'a self::Config) -> Result<self::Intermediate<'a>, self::Error> { // TODO: Change this soon
		let mut intermediate: self::Intermediate<'a> = self::Intermediate::new(config)?;
		let mut consumers: Vec<Consumer> = Vec::new();
		
		for mod_ in &config.mods {
			let get_chains_result: ::mlua::Result<Vec<Chain>> = mod_.lua.globals().get_path::<Vec<Chain>>("__Z_SHARP__.__UNSAFE__.registry.chains");
			
			match get_chains_result {
				Ok(chains_gotten) => {
					consumers.append(
						&mut chains_gotten.iter()
							.map(|chain: & Chain| {
								return chain.create_consumer();
							})
							.collect::<Vec<_>>()							
					);
				},
				Err(_) => {
					// # The __Z_SHARP__ internal proxy has failed.
					// This is a serious user error.
					
					return Err(self::Error::LuauError);
				},
			};
		}

		// info!("Building... {:#?}", consumers);

		intermediate.consumers = consumers;

		return Ok(intermediate);
	}
}