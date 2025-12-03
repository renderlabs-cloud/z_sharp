

local Object = { };
export type Object<T> = {
	value: T,
	meta: any?,

	[any]: any,

	unwrap: (self: Object<T>) -> T,
};


function Object.from<T>(value: T): Object<T>
	local internal: Object<T> = {
		value = value,

		unwrap = function(self)
			return self.value;
		end,
	};

	local proxy: any = {};

	local metatable: any = {
		__index = function(self, key)
			local value: any = type(internal.value) == 'table' and internal.value[key] or internal[key]; -- Override value.

			return value;
		end,
	};

	metatable.__newindex = function(self, key, value)
		if key == 'meta' then
			internal.meta = value;
			-- Combine metatables.
			local new_metatable: any = {
				__index = value.__index or metatable.__index,
				__newindex = value.__newindex or metatable.__newindex,
			};

			for key, new_value in pairs(value) do
				new_metatable[key] = new_value;
			end;

			setmetatable(proxy, new_metatable);
			return;
		end;

		internal[key] = value;
	end;
	
	setmetatable(proxy, metatable);

	return proxy :: any;
end;

function Object:unwrap<T>(object: Object<T>): T
	return object.value;
end;

--- TODO: Implement `Array`.
export type Array<T> = {
	[number]: Object<T>,
};

local class: Object<any> = Object.from({});
export type Class<C...> = Object<{
	prototype: Object<{
		[any]: any,
	}>;

	constructor: (any, C...) -> (nil);
}>;

export type Construct<C...> = {
	constructor: (any, C...) -> (nil);
	[any]: Object<any>;
};

class.meta = {
	__call = function <T...>(self, extendant: Class<any>?): (Construct<T...>) -> Class<T...>
		return function (construct: Construct<T...>): Class<T...>
			local object: Class<T...> = Object.from({
				prototype = Object.from({
					super = function(...)
						if not extendant then
							return;
						end;

						return extendant['constructor'](self, ...);
					end,
				}),

				constructor = function(self, ...) -- Intermediate constructor.
					construct.constructor(self, ...);
				end,
			});

			object.meta = {
				__call = function(...) -- Final constructor.
					--object.constructor(object, )
				end,
			};

			return object;
		end
	end,
};

return {
	Object = Object,
	
	class = class,
};