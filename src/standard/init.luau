local __DARKLUA_BUNDLE_MODULES={cache={}::any}do do local function __modImpl()local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Test: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('test');
		local details = lexer.RuleDetails.Single(
			lexer.SingleDetails.new({
				pattern = 'test',
			})
		);

		chain:capture(lexer.Rule.new({
			details = details,
			name = 'test',
			required = true,
		}));

		return chain	
end)(),
};

return Test end function __DARKLUA_BUNDLE_MODULES.a():typeof(__modImpl())local v=__DARKLUA_BUNDLE_MODULES.cache.a if not v then v={c=__modImpl()}__DARKLUA_BUNDLE_MODULES.cache.a=v end return v.c end end end--[[
	Currently this file is for testing.
	In the future the file(s) will be the standard.

	Anyways, thanks for checking this project out!

	TODO: Compile this BEFORE loading into Z#. I'm not sure how to do this yet.

	@author Brendan Lucas.
--]]


local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;
local console = z_sharp.console;

-- Register these chains.
local chains = {
	Test = __DARKLUA_BUNDLE_MODULES.a(),
	-- Structure = require('@src/features/structure.luau'),

	-- Function = require('@src/features/function.luau'),
	
	-- Whitespace = require('@src/features/whitespace.luau'),
};

for _, chain in pairs(chains) do
	lexer.register_chain(chain.chain);
end;

console.log('Registered chains.');

-- _G.__Z_SHARP__.__UNSAFE__.registry = nil; -- Sabotage the Luau proxy for testing purposes.