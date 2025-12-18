pub mod error;
pub mod intermediate;
pub mod source;

use crate::modification::Modification;

pub struct Config {
	pub mods: Vec<Modification>,
	// ...
}
