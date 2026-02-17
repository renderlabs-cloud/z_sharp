// Rename this file?

use ::mlua::Value;

pub fn to_string(value: &Value) -> String {
	let result: Result<String, ::serde_json::Error> = ::serde_json::to_string_pretty(value);

	return match result {
		Ok(s) => s,
		Err(_) => {
			match value.to_string() {
				Ok(s) => s,
				Err(e) => format!("<\"{}\">", e),
			}
		},
	};
}
