pub mod my_mod;

#[macro_use]
extern crate time_test;

#[macro_use]
extern crate unwrap;

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

	use crate::my_mod;

	use ::tokio::{ fs, };

	#[tokio::test]
	async fn zero() -> Result<(), builder::Error> {
		time_test!();
		::tracing_subscriber::fmt::init();

		let my_mod_result: Result<Modification, ::mlua::Error>  = my_mod::get();

		match my_mod_result {
			Ok(_) => {

			},
			Err(err) => {
				panic!();
			}
		};

		let binding: builder::Config = builder::Config {
			mods: vec![
				unwrap!(my_mod_result),
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