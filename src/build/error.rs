use crate::build::source;

use std::any::TypeId;
use ::std::fmt;

use ::rust_i18n::t;

// TODO: Refactor into groups?
#[derive(Debug)]
pub enum Error {
	NoConsumers,
	NoMatches(source::Position),
	NoSourceResolversAvailable,
	NoSourceResolversAvailableFor(String),

	LuauError(::mlua::Error),
	ProxyFailure(::mlua::Error),
	ConversionError(TypeId, TypeId),
}

impl fmt::Display for self::Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return match self {
			Error::NoConsumers => write!(f, "{}", t!("Luau.errors.no_consumers")),
			Error::ProxyFailure(error) => {
				write!(f, "{}", t!("Luau.errors.proxy_failure", error = error))
			},
			Error::LuauError(error) => {
				write!(f, "{}", t!("Luau.errors.runtime", error = error))
			},
			_ => write!(f, "{:?}", self),
		};
	}
}

impl std::error::Error for self::Error {
}
