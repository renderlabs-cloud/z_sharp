--!strict

local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local patterns = require('@src/patterns.luau');

local Whitespace = require('@src/features/whitespace.luau');
local Type_Reference = require('@src/features/type_reference.luau');

local Structure: z_sharp.ChainImpl = {
	chain = (function()
		local chain = lexer.Chain.new('structure');

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = 'struct',
					required = true,
				})
			)	
		);

		chain:capture(
			lexer.Rule.Child(
				lexer.ChildConfig.new({
					child = Whitespace.chain,
					required = true,
				})
			)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = 'identifier',
					pattern = patterns.IDENTIFIER,
					required = true,
				})
			)
		);

		chain:capture(
			lexer.Rule.Child(
				lexer.ChildConfig.new({
					child = Whitespace.chain,
					required = false,
				})
			)
		);

		chain:capture(
			lexer.Rule.Child(
				lexer.ChildConfig.new({
					child = Type_Reference.chain,
					required = true,
				})
			)
		);

		return chain;
	end)(),
	child = function ()
		return {} :: any;
	end,
};

return Structure;