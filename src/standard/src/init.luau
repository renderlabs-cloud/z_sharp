--!strict
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
	Function = require('@self/chains/function.luau'),
	
	Whitespace = require('@self/chains/whitespace.luau'),
};

for _, chain in pairs(chains) do
	lexer.register_chain(chain.chain);
end;

_G.__Z_SHARP__.__UNSAFE__.registry = nil; -- Sabotage the Luau proxy for testing purposes.