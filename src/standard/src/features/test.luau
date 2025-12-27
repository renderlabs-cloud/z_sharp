local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Test: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('test');
		local details = lexer.RuleDetails.Single(
			lexer.SingleDetails.new({
				pattern = 'test',
			})
		);

		chain:capture(lexer.Rule.new({
			details = details,
			name = 'test',
			required = true,
		}));

		return chain;
	end)(),
};

return Test;