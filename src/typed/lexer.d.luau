export type Chain = {
	capture: (Rule) -> (),
};

export type Rule = {
	Single: (SingleConfig) -> Rule,
	Or: (OrConfig) -> Rule,
	Repeat: (RepeatConfig) -> Rule,
	Child: (ChildConfig) -> Rule,
	Logic: (LogicConfig) -> Rule,
};

local Rule = {

};

export type SingleConfig = {
	name: string?,
	pattern: string,
	required: boolean?,
};

local SingleConfig: any = {
	-- new = function ()
};

export type OrConfig = {
	name: string?,
	rules_list: { Rule },
	required: boolean?,
};

export type RepeatConfig = {
	name: string,
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

local lexer = {

	Rule = Rule,
	Chain = {
		new = function (name): Chain
			return {} :: Chain;
		end,
	},
	register_chain = function (chain: Chain): () end,
};

return lexer;