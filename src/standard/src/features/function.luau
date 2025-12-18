--!strict

local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local patterns = require('@src/patterns.luau');

local Whitespace = require('@src/features/whitespace.luau');

local Function: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('function');
			
		-- TODO: Convert to list form?
		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = 'func',
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
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = '\\(',
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

		-- TODO: Function parameters.

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = '\\)',
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
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = '\\:',
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
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = patterns.IDENTIFIER, -- TODO: Replace with type.
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

		-- TODO: Use section instead of `{` and `}`.
		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = '\\{',
					required = true,
				})
			)
		);

		-- TODO: Function body.

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					pattern = '\\}',
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

return Function;