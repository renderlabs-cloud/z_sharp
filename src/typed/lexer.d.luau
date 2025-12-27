--[[
	Type declarations for the lexer.
]]

local user_data = require('@src/user_data.luau');

export type SingleDetails = {
	pattern: string,
};

local SingleDetails = {
	new = function (_: SingleDetails): user_data.Of<SingleDetails>
		return {} :: user_data.Of<SingleDetails>;
	end
};

export type OrDetails = {
	rules_map: { [string]: { Rule } },
};

local OrDetails = {
	new = function (_: OrDetails): user_data.Of<OrDetails>
		return {} :: user_data.Of<OrDetails>;
	end
};

export type RepeatDetails = {
	rules: { Rule },
	min: number?,
	max: number?,
};

local RepeatDetails = {
	new = function (_: RepeatDetails): user_data.Of<RepeatDetails>
		return {} :: user_data.Of<RepeatDetails>;
	end
};

export type ChildDetails = {
	child: Chain,
};

local ChildDetails = {
	new = function (_: ChildDetails): user_data.Of<ChildDetails>
		return {} :: user_data.Of<ChildDetails>;
	end
};

-- TODO: Clarify types.
export type LogicDetails = {
	func: ({ [string]: any }) -> boolean,
};

local LogicDetails = {
	new = function (_: LogicDetails): user_data.Of<LogicDetails>
		return {} :: user_data.Of<LogicDetails>;
	end
};

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

export type RuleDetails = {
	Single: (SingleDetails) -> RuleDetails,
	Or: (OrDetails) -> RuleDetails,
	Repeat: (RepeatDetails) -> RuleDetails,
	Child: (ChildDetails) -> RuleDetails,
	Logic: (LogicDetails) -> RuleDetails,
};

local RuleDetails = {
	Single = function (_: user_data.Of<SingleDetails>): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	Or = function (_: user_data.Of<OrDetails>): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	Repeat = function (_: user_data.Of<RepeatDetails>): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	Child = function (_: user_data.Of<ChildDetails>): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	Recurse = function (): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	Logic = function (_: user_data.Of<LogicDetails>): user_data.Of<RuleDetails>
		return {} :: user_data.Of<RuleDetails>;
	end,
	None = {},
};

export type Chain = {
	name: string,
	rules: { Rule },
	
	capture: (self: Chain, Rule) -> (),
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
	SingleDetails = SingleDetails,
	RepeatDetails = RepeatDetails,
	OrDetails = OrDetails,
	ChildDetails = ChildDetails,
	LogicDetails = LogicDetails,

	Rule = Rule,
	RuleDetails = RuleDetails,

	Chain = Chain,
	register_chain = function (chain: Chain): () end,
};

return lexer;