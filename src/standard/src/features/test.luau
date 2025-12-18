--!strict
--[=[

--]=]
local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Test: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('test');
		
		chain:capture({
			details = lexer.RuleDetails.Single({
				pattern = '',
				required = true,
			}),
			name = 'test',
		});

		return chain;
	end)(),
};

return Test;