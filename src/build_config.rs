use ::std::collections::HashMap;

use crate::modification::Modification;

pub struct BuildConfig {
	pub mods: Vec<Modification>,
	pub source: HashMap<String, String>,
	// ...
}