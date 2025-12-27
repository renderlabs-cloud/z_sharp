
local sugar = require('@src/sugar.luau'); 

--[=[
	UserData.
]=]
export type Of<T> = T & sugar.Is<'UserData'>;

return {};