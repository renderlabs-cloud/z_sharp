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

export type SingleConfig = {
	name: string?,
	pattern: string,
	required: boolean?,
};

export type OrConfig = {
	name: string?,
	rules_list: { Rule },
	required: boolean?,
};

export type RepeatConfig = {
	name: string?,
	rules: { Rule },
	min: number?,
	max: number?,
	required: boolean?,
};

export type ChildConfig = {
	name: string?,
	child: Chain,
	required: boolean?,
};

export type LogicConfig = {
	func: () -> boolean,
};

export type Rule = {
	Single: (SingleConfig) -> any,
	Or: (OrConfig) -> any,
	Repeat: (RepeatConfig) -> any,
	Child: (ChildConfig) -> any,
	Logic: (LogicConfig) -> any,
};

export type Chain = {
	capture: (Rule) -> (),
};

z_sharp.Chain = {
	new = function (name): Chain
		return {} :: Chain;
	end,
};

return z_sharp;