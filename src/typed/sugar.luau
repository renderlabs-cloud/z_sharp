
--[=[
	This is a stub type. It acts as a lock to differentiate between a type and a super type.

	
]=]
export type Is<I> = {
	_: <I>() -> (),
};

return {};