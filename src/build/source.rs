use ::std::{fmt, pin::Pin};

pub type SourceResolverKernel = dyn Future<Output = Option<String>> + Send;

pub type SourceResolver = Box<dyn Fn(String) -> Pin<Box<SourceResolverKernel>> + Send + Sync>;

#[derive(Default, Debug)]
pub struct Position {
	pub line: u32,
	pub column: u32,
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		return write!(f, "{}:{}", self.line, self.column);
	}
}
