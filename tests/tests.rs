pub mod my_mod;

#[macro_use]
extern crate time_test;

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

#[cfg(test)]
mod tests {
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

		let my_mod_result: Result<Modification, mlua::Error>  = my_mod::get();

		match my_mod_result {
			Ok(_) => {

			},
			Err(err) => {
				panic!("{}", err);
			}
		};

		let binding: builder::Config = builder::Config {
			mods: vec![my_mod_result.unwrap()],
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

		intermediate.interpret(self::TEST_SCRIPT).await?;

		return Ok(());
	}
}