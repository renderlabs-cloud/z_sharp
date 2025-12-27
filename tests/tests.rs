pub mod standard;

#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
	// HACK (+) Don't recompile for changes in `test.zs`.
	const TEST_SCRIPT: &str = "tests/test.zs";

	use ::z_sharp::{
		build::{Config, error::Error, intermediate::Intermediate},
		modification::Modification,
	};

	use crate::standard;

	use ::tokio::fs;

	async fn main() -> Result<(), Error> {
		time_test!();
		::tracing_subscriber::fmt::init();
		::log::set_max_level(::log::LevelFilter::Trace);

		let my_mod: Modification = match standard::get() {
			| Ok(mod_) => mod_,
			| Err(error) => {
				return Err(Error::LuauError(error));
			},
		};

		let binding: Config = Config { mods: vec![my_mod] };

		let mut intermediate: Intermediate = Intermediate::new(&binding)?;

		// FS source resolver.
		intermediate.add_source_resolvers(Box::new(|path: String| {
			return Box::pin(async move {
				let contents: Option<String> = fs::read_to_string(path).await.ok();

				return contents;
			});
		}));

		intermediate.request_source(self::TEST_SCRIPT).await?;

		return intermediate.interpret(self::TEST_SCRIPT).await;
	}

	#[tokio::test]
	async fn wrap() -> () {
		let result: Result<(), ::z_sharp::build::error::Error> = self::main().await;

		match result {
			| Ok(()) => {},
			| Err(error) => {
				::log::error!("{}", error);
				panic!();
			},
		};
	}
}
