#[macro_export]
macro_rules! lua_table {
	($name:ident { $($field:ident : $t:ty),* $(,)? }) => {
		#[derive(Clone, Debug)]
		#[mlua_magic_macros::structure]
		pub struct $name {
			$(pub $field: $t,)*
		}

		#[mlua_magic_macros::implementation]
		impl $name {
			pub fn new(table: Table) -> Result<Self, ::mlua::Error> {
				// ::log::info!("Creating new {}: {:#?}", stringify!($name), table);

				return Ok(Self {
					$($field: table.get(stringify!($field))?,)*
				});
			}
		}

		mlua_magic_macros::compile!(type_path = $name, fields = true, methods = true);
	};
}

pub use crate::lua_table;
