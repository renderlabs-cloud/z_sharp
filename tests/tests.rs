pub mod my_mod;

#[macro_use]
extern crate time_test;

#[macro_use]
extern crate tracing;

#[cfg(test)]
mod tests {
	use ::z_sharp::builder;

	use crate::my_mod;

	use ::tokio::{ fs, };

	#[tokio::test]
	async fn zero() -> Result<(), builder::Error> {
		time_test!();
		::tracing_subscriber::fmt::init();

		let binding: builder::Config = builder::Config {
			mods: vec![my_mod::get()],
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

		intermediate.request_source("tests/test.zs".to_string()).await?;

		intermediate.interpret().await?;

		info!("Done!");

		return Ok(());
	}
}