--[=[
	Type declarations for the lexer.
]=]

--[=[
	`capture.rs`.
]=]

-- TODO: Remove the need for UserData wrappers.

export type SingleDetails = {
	pattern: string,
};

local SingleDetails = {
	new = function (_: SingleDetails): SingleDetails
		return {} :: SingleDetails;
	end
};

export type OrDetails = {
	rules_map: { [string]: { Rule } },
};

local OrDetails = {
	new = function (_: OrDetails): OrDetails
		return {} :: OrDetails;
	end
};

export type RepeatDetails = {
	rules: { Rule },
	min: number?,
	max: number?,
};

local RepeatDetails = {
	new = function (_: RepeatDetails): RepeatDetails
		return {} :: RepeatDetails;
	end
};

export type ChildDetails = {
	child: Chain,
};

local ChildDetails = {
	new = function (_: ChildDetails): ChildDetails
		return {} :: ChildDetails;
	end
};

--[=[
	@TODO Clarify types.
]=]
export type LogicDetails = {
	func: ({ [string]: any }) -> boolean,
};

local LogicDetails = {
	new = function (_: LogicDetails): LogicDetails
		return {} :: LogicDetails;
	end
};

export type Rule = {
	name: string?,
	required: boolean?,
	details: RuleDetails,
};

export type RuleDetails = {
	Single: (SingleDetails) -> RuleDetails,
	Or: (OrDetails) -> RuleDetails,
	Repeat: (RepeatDetails) -> RuleDetails,
	Child: (ChildDetails) -> RuleDetails,
	Logic: (LogicDetails) -> RuleDetails,
};

local RuleDetails = {
	Single = function (_: SingleDetails): RuleDetails
		return {} :: RuleDetails;
	end,
	Or = function (_: OrDetails): RuleDetails
		return {} :: RuleDetails;
	end,
	Repeat = function (_: RepeatDetails): RuleDetails
		return {} :: RuleDetails;
	end,
	Child = function (_: ChildDetails): RuleDetails
		return {} :: RuleDetails;
	end,
	Recurse = function (): RuleDetails
		return {} :: RuleDetails;
	end,
	Logic = function (_: LogicDetails): RuleDetails
		return {} :: RuleDetails;
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

	-- Add rule?
	RuleDetails = RuleDetails,

	Chain = Chain,
	register_chain = function (chain: Chain): () end,
};

return lexer;