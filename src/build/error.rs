use crate::build::source;

use ::std::fmt;

use ::rust_i18n::t;

use ::mlua;

#[derive(Debug)]
pub enum Error {
	NoConsumers,
	NoMatches(source::Position),
	NoSourceResolversAvailable,
	NoSourceResolversAvailableFor(String),
	LuauError(mlua::Error),
	ProxyFailure(mlua::Error),
}

impl fmt::Display for self::Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		match self {
			| Error::NoConsumers => write!(f, "{}", t!("Luau.errors.no_consumers")),
			| Error::ProxyFailure(error) => {
				write!(f, "{}", t!("Luau.errors.proxy_failure", error = error))
			},
			| Error::LuauError(error) => {
				write!(f, "{}", t!("Luau.errors.runtime_failure", error = error))
			},
			| _ => write!(f, "{:?}", self),
		}
	}
}

impl std::error::Error for self::Error {
}
