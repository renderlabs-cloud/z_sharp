--!strict

local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Whitespace = require('@src/features/whitespace.luau');

local Structure: z_sharp.ChainImpl = {
	chain = (function()
		local chain = lexer.Chain.new('structure');

		-- z_sharp.console.log(lexer.ChildDetails.new({child = Whitespace.chain,}))

		chain:capture_all({
			lexer.Rule.new({
				details = lexer.RuleDetails.Single(
					lexer.SingleDetails.new({
						pattern = 'struct',
					})
				),
				required = true,
			}),
			--[[lexer.Rule.new({
				details = lexer.RuleDetails.Child(
					lexer.ChildDetails.new({
						child = Whitespace.chain,
					})
				),
				required = false,
			}),]]
		});

		--[[chain:capture(
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
		);]]

		return chain;
	end)(),
	child = function ()
		return {} :: any;
	end,
};

return Structure;