pub mod my_mod;

#[allow(unused_imports)]
use ::tracing_subscriber;
#[deny(unused_imports)]

#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
    use ::z_sharp::BuildConfig;
	use crate::my_mod;

	use ::std::collections::HashMap;

	#[test]
	fn zero() {
		time_test!();
		tracing_subscriber::fmt::init();
		z_sharp::build(
			BuildConfig {
				mods: vec![my_mod::get()],
				source: HashMap::from([
					("test.zs".to_string(), include_str!("test.zs").to_string())
				])
			}
		).unwrap();
	}
}