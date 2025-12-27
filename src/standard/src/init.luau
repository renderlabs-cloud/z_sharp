--[[
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
	Test = require('@src/features/test.luau'),
	-- Structure = require('@src/features/structure.luau'),

	-- Function = require('@src/features/function.luau'),
	
	-- Whitespace = require('@src/features/whitespace.luau'),
};

for _, chain in pairs(chains) do
	lexer.register_chain(chain.chain);
end;

console.log('Registered chains.');

-- _G.__Z_SHARP__.__UNSAFE__.registry = nil; -- Sabotage the Luau proxy for testing purposes.