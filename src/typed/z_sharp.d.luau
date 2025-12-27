--[[
	Type declarations for the Z# internal library.
	This file does not contain any functionality.

	@author Brendan Lucas
--]]

local z_sharp = {};

z_sharp.console = {
	log = function (message): () end,
	warn = function (message): () end,
	error = function (message): () end,
};

local lexer = require('@src/lexer.d.luau');

export type ChainImpl = lexer.ChainImpl;

z_sharp.lexer = lexer;

return z_sharp;