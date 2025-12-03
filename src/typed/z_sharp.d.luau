--!strict
--[[
-- # Type declarations for the Z# internal library.
---- @Author Brendan Lucas
----
-- ? This file does not contain any functionality.
--]]

local z_sharp = {};

-- # console
z_sharp.console = {
	log = function (message): () end,
	warn = function (message): () end,
	error = function (message): () end,
};

-- # lexer
z_sharp.lexer = {};

type SingleConfig = {
	name: string?,
	pattern: string,
	required: boolean?,
};

type OrConfig = {
	name: string?,
	rules_list: { Rule },
	required: boolean?,
};

type RepeatConfig = {
	name: string?,
	rules: { Rule },
	min: number?,
	max: number?,
};

type LogicConfig = {
	name: string?,
	func: () -> boolean,
};

type Rule = {
	Single: (SingleConfig) -> any,
	Or: (OrConfig) -> any,
	Repeat: (RepeatConfig) -> any,
	Logic: (LogicConfig) -> any,
};

type Chain = {
	capture: (Rule) -> (),
};

z_sharp.Chain = {
	new = function (name): Chain
		return {} :: Chain;
	end,
};

