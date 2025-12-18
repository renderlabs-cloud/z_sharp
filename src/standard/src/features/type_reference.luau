--!strict

local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;
local console = z_sharp.console;

local patterns = require('@src/patterns.luau');

local Whitespace = require('@src/features/whitespace.luau');

local Type_Reference: z_sharp.ChainImpl = {
	chain = (function()
		local chain = lexer.Chain.new('type_reference');
		
		chain:capture(
			lexer.Rule.Or(
				lexer.OrConfig.new({
					name = 'declaration',
					rules_map = {
						['fields'] = {
							[1] = lexer.Rule.Single(
								lexer.SingleConfig.new({
									pattern = '\\{',
									required = true,
								})
							),
							[2] = lexer.Rule.Repeat(
								lexer.RepeatConfig.new({
									name = 'fields',
									rules = {
										[1] = lexer.Rule.Child(
											lexer.ChildConfig.new({
												child = Whitespace.chain,
												required = false,
											})
										),
										[2] = lexer.Rule.Single(
											lexer.SingleConfig.new({
												name = 'label',
												pattern = patterns.IDENTIFIER,
											})
										),
										[3] = lexer.Rule.Child(
											lexer.ChildConfig.new({
												child = Whitespace.chain,
												required = false,
											})
										),
										[4] = lexer.Rule.Single(
											lexer.SingleConfig.new({
												pattern = ':',
											})
										),
										[5] = lexer.Rule.Child(
											lexer.ChildConfig.new({
												child = Whitespace.chain,
												required = false,
											})
										),
										[6] = lexer.Rule.Recurse(
											lexer.RecurseConfig.new({
												name = 'type',
												required = true,
											})
										),
									},
									required = false,
								})
							),
							[3] = lexer.Rule.Single(
								lexer.SingleConfig.new({
									pattern = '\\}',
									required = true,
								})
							),
						}
					},
					required = true,
				})
			)
		);

		return chain;
	end)(),
};

return Type_Reference;