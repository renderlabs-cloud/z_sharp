pub mod standard;

#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
	// HACK (+) Don't recompile for changes in `test.zs`.
	const TEST_SCRIPT: & str = "tests/test.zs";


	use ::z_sharp::{
		builder,
		modification::{
			Modification,
		},
	};

	use crate::standard;

	use ::tokio::{ fs, };

	#[tokio::test]
	async fn zero() -> Result<(), builder::Error> {
		time_test!();
		::tracing_subscriber::fmt::init();

		let my_mod: Modification = match standard::get() {
			Ok(mod_) => mod_,
			Err(err) => {
				::log::error!("{:#?}", err);
				return Err(builder::Error::LuauError);
			},
		};

		let binding: builder::Config = builder::Config {
			mods: vec![
				my_mod,
			],
		};

  		let mut intermediate: builder::Intermediate = builder::new(
			&binding
		)?;

		intermediate.add_source_resolvers(Box::new(|path: String| {
			return Box::pin(async move {
				let contents: Option<String> = fs::read_to_string(path).await.ok();

				return contents;
			});
		}));

		intermediate.request_source(self::TEST_SCRIPT).await?;

		let result: Result<(), builder::Error> = intermediate.interpret(self::TEST_SCRIPT).await;

		match result {
			Ok(_) => { },
			Err(err) => {
				::log::error!("{:#?}", err);
				panic!();
			},
		};

		return Ok(());
	}
}