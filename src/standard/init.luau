local __DARKLUA_BUNDLE_MODULES={cache={}::any}do do local function __modImpl()--!strict
--[=[

--]=]
local z_sharp = require('@z_sharp');

local Whitespace = {
	chain = (function ()
		local chain: z_sharp.Chain = z_sharp.lexer.Chain.new('whitespace');
		

		chain:capture(
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
					name = nil,
					pattern = '\\s+',
					required = true,
				})
			)
		);

		chain:done();

		return chain	
end)(),
};

function Whitespace.child(required: boolean): ChildConfig
	return z_sharp.lexer.Rule.Child(
		z_sharp.lexer.ChildConfig.new({
			name = nil,
			child = Whitespace.chain,
			required = required,
		})
	) :: ChildConfig
end;

return Whitespace end function __DARKLUA_BUNDLE_MODULES.a():typeof(__modImpl())local v=__DARKLUA_BUNDLE_MODULES.cache.a if not v then v={c=__modImpl()}__DARKLUA_BUNDLE_MODULES.cache.a=v end return v.c end end do local function __modImpl()--!strict
--[=[
	Function declatation.
--]=]


local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local patterns = {
	IDENTIFIER = '[a-zA-Z_$][a-zA-Z0-9_$]+',
}; -- require('patterns.luau'); -- Load local files.

local Whitespace = __DARKLUA_BUNDLE_MODULES.a();

local Function = {
	chain = (function ()
		local chain = lexer.Chain.new('function');
		
		chain:capture(
			lexer.Rule.Single({
				name = nil,
				pattern = 'function',
				required = true,
			})
		);

		chain:capture(
			Whitespace.child(true)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = 'identifier',
					pattern = patterns.IDENTIFIER,
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\(',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		-- TODO: Function parameters.

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\)',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\:',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = patterns.IDENTIFIER, -- TODO: Replace with type.
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		-- TODO: Use section instead of `{` and `}`.
		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\{',
					required = true,
				})
			)
		);

		-- TODO: Function body.

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\}',
					required = true,
				})
			)
		);
		
		chain:capture(
			lexer.Rule.Logic(
				lexer.LogicConfig.new({
					func = function (results): boolean
						z_sharp.console.log(results.identifier.data);
						return true					
end,
				})
			)
		);

		chain:done();

		return chain	
end)(),
};

return Function end function __DARKLUA_BUNDLE_MODULES.b():typeof(__modImpl())local v=__DARKLUA_BUNDLE_MODULES.cache.b if not v then v={c=__modImpl()}__DARKLUA_BUNDLE_MODULES.cache.b=v end return v.c end end do local function __modImpl()--!strict
--[=[

--]=]

local z_sharp = require('@z_sharp');

local Whitespace = {
	chain = (function ()
		local chain: z_sharp.Chain = z_sharp.lexer.Chain.new('whitespace');
		

		chain:capture(
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
					name = nil,
					pattern = '\\s+',
					required = true,
				})
			)
		);

		chain:done();

		return chain	
end)(),
};

function Whitespace.child(required: boolean): ChildConfig
	return z_sharp.lexer.Rule.Child(
		z_sharp.lexer.ChildConfig.new({
			name = nil,
			child = Whitespace.chain,
			required = required,
		})
	) :: ChildConfig
end;

return Whitespace end function __DARKLUA_BUNDLE_MODULES.c():typeof(__modImpl())local v=__DARKLUA_BUNDLE_MODULES.cache.c if not v then v={c=__modImpl()}__DARKLUA_BUNDLE_MODULES.cache.c=v end return v.c end end end--!strict
--[=[
	Currently this file is for testing.
	In the future the file(s) will be the standard.

	Anyways, thanks for checking this project out!
	TODO: Compile this BEFORE loading into Z#. I'm not sure how to do this yet.

	@Author Brendan Lucas
--]=]


local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

-- Register these chains.
local chains = {
	Function = __DARKLUA_BUNDLE_MODULES.b(),
	
	Whitespace = __DARKLUA_BUNDLE_MODULES.c(),
};

for _, chain in pairs(chains) do
	lexer.register_chain(chain.chain);
end;

_G.__Z_SHARP__.__UNSAFE__.registry = nil; -- Sabotage the Luau proxy for testing purposes.