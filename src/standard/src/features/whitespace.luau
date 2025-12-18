--!strict
--[=[

--]=]
local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local Whitespace: z_sharp.ChainImpl = {
	chain = (function ()
		local chain = lexer.Chain.new('whitespace');
		
		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\s+',
					required = true,
				})
			)
		);

		return chain;
	end)(),
};

return Whitespace;