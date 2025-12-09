// TODO: This will eventually become the standard.

use ::z_sharp::modification;
use z_sharp::modification::Modification;

pub fn get() -> ::mlua::Result<Modification> {
	return modification::lua::new(
		"My Mod", 
		include_str!("../src/standard/init.luau").to_string()
	);
}