use ::static_toml::static_toml;

// # Version config

static_toml! {
	pub static VERSION = include_toml!("src/version.toml");
}

// # Localization config

i18n!("i18n", fallback = "en");