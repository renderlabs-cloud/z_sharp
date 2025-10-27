use ::static_toml::static_toml;

static_toml! {
	pub static VERSION = include_toml!("src/version.toml");
}