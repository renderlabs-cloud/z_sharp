local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Test: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('test');

		chain:capture_all({
			lexer.Rule.new({
				type = 'single',
				details = {
					pattern = 'test',
				},
				name = 'test',
				required = true,
			}),
		});

		return chain;
	end)(),
};

return Test;