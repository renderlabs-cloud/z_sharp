pub mod builder {
	use crate::modification::{
			Modification,
			capture::Chain,
			consumer::{
				Consumer
			},
			// gluea::{ Gluea, },
		};

	use ::std::collections::{ HashMap, };
	use ::std::future::{ Future, };
	use ::std::pin::{ Pin, };

	use ::derive_more::with_trait::{
		Display,
	};
	use ::mlua::{
		Value, Table,
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

	impl<'a> Intermediate<'a> {
		pub fn new(config: &'a self::Config) -> Result<Self, self::Error> {
			let intermediate: Self = Self {
				config: config,
				sources: HashMap::new(),
				source_resolvers: Vec::new(),
    			consumers: Vec::new(),
			};
			
			return Ok(intermediate);
		}

		pub fn add_source_resolvers(& mut self, source_resolver: self::SourceResolver) -> () {
			self.source_resolvers.push(source_resolver);
		}

		pub async fn request_source(& mut self, name: String) -> Result<(), self::Error> {
			let mut set: tokio::task::JoinSet::<Option<String>> = tokio::task::JoinSet::new();

			for source_resolver in 	& self.source_resolvers {
				set.spawn(source_resolver(name.clone()));
			};
			
			if set.is_empty() {
				return Err(self::Error {
					message: format!("Could not resolve source \"{}\"", name).to_string(), // TODO: Localization.
				});
			};

			while let Some(join_result) = set.join_next().await {
				match join_result {
					Ok(source_option) => {
						if let Some(source) = source_option {
							self.sources.insert(name, source);
							return Ok(()); // Context gets destroyed since the race was successful.
						};
 					},
					Err(err) => {
						error!("{}", err);
						todo!();
					},
				};
			};

			return Ok(());
		}

		pub async fn interpret(& mut self) -> Result<(), self::Error> {
			// TODO: Blocks, ...etc.
			// TODO: Optimize.

			for source in self.sources.values() {
				info!(source);

				let mut remaining: & str = source;

				while !remaining.is_empty() {
					let last_length: usize = remaining.len();

					for ref mut consumer in &mut self.consumers {
						remaining = consumer.consume(remaining);
					};

					if remaining.len() == last_length {
						// # No progress was made.
						// TODO: Add error handling

						error!("Remaining text!: {}", remaining);
						todo!();
					};
				};
			};

			return Ok(());
		}
	}

	// TODO: Make error type printable.
	#[derive(Debug)]
	pub struct Error {
		message: String,
	}

	impl Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			return write!(f, "{}", self.message);
		}
	}

	pub fn new<'a>(config: &'a self::Config) -> Result<self::Intermediate<'a>, self::Error> { // TODO: Change this soon
		let mut intermediate: self::Intermediate<'a> = self::Intermediate::new(config)?;
		let mut consumers: Vec<Consumer> = Vec::new();
		
		for mod_ in &config.mods {
			let get_chains_result: mlua::Result<Vec<Chain>> = mod_.lua.globals().get_path("__Z_SHARP__.__UNSAFE__.__EXPORTS__.chains");
			
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
				Err(err) => {
					// # The __Z_SHARP__ internal proxy has failed.
					// This is either a bug, or a user error.
					// TODO: Update logging methods.
					
					error!("{}", err);
					todo!(); // TODO: Add error handling.
				},
			};
		}

		info!("Building... {:#?}", consumers);

		intermediate.consumers = consumers;

		return Ok(intermediate);
	}
}