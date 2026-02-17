--[[
	Type declarations for the lexer.
]]

local user_data = require('@src/user_data.luau');

export type SingleDetails = {
	pattern: string,
};

export type OrDetails = {
	rules_map: { [string]: { Rule } },
};

export type RepeatDetails = {
	rules: { Rule },
	min: number?,
	max: number?,
};

export type ChildDetails = {
	child: Chain,
};

export type RecurseDetails = {

};

-- TODO: Clarify types.
export type LogicDetails = {
	func: ({ [string]: any }) -> boolean,
};


export type RuleDetails = SingleDetails | OrDetails | RepeatDetails | ChildDetails | RecurseDetails | LogicDetails;


export type Rule = {
	name: string?,
	required: boolean?,
	details: RuleDetails,
};

local Rule = {
	new = function (_: Rule): user_data.Of<Rule>
		return {} :: user_data.Of<Rule>;
	end
};

export type Chain = {
	name: string,
	rules: { Rule },
	
	capture: (self: Chain, Rule) -> (),
	capture_all: (self: Chain, rules: { Rule }) -> (),
};

local Chain = {
	new = function (_: string): Chain
		return {} :: Chain;
	end,
};

export type ChainImpl = {
	chain: Chain,
};

-- ...

local lexer = {
	Rule = Rule,

	Chain = Chain,
	register_chain = function (chain: Chain): () end,
};

return lexer;