--!strict
--[=[
	Type declarations for the Z# internal library.
	This file does not contain any functionality.

	@Author Brendan Lucas
--]=]

local z_sharp = {};

-- # console
z_sharp.console = {
	log = function (message): () end,
	warn = function (message): () end,
	error = function (message): () end,
};

-- # lexer
local lexer = require('@src/lexer.d.luau');

z_sharp.lexer = lexer;

return z_sharp;