/*!
 *! # Z#
 *! 
*/

pub mod build_config;
pub mod modification;
pub mod version;

use ::std::fmt::Error;

pub use crate::build_config::BuildConfig;

// Dev
use ::tracing::{info};

pub fn build(config: BuildConfig) -> Result<i32, Error> { // TODO: Change this soon
	info!("Building...");

	return Ok(1);
}