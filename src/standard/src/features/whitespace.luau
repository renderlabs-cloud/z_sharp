--[=[

--]=]
local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local console = z_sharp.console;

local Whitespace: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('whitespace');

		console.log(
			lexer.Rule.new({
				details = lexer.RuleDetails.Single(
					lexer.SingleDetails.new({
						pattern = '\\s+',
					})
				),
				name = 'whitespace',
				required = true,
			})
		);
		
		chain:capture_all({
			lexer.Rule.new({
				details = lexer.RuleDetails.Single(
					lexer.SingleDetails.new({
						pattern = '\\s+',
					})
				),
				name = 'whitespace',
				required = true,
			})
		});

		return chain;
	end)(),
};

return Whitespace;