use z_sharp::modification;
use z_sharp::modification::Modification;

pub fn get() -> Modification {
	return modification::lua::new(
		"My Mod", 
		include_str!("core.lua").to_string()
	).unwrap();
}