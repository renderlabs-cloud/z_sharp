use ::z_sharp::modification;
use z_sharp::modification::Modification;

pub fn get() -> ::mlua::Result<Modification> {
	return modification::lua::new(
		"My Mod", 
		include_str!("../src/standard/core.luau").to_string()
	);
}