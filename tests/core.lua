local z_sharp = require('z_sharp');

local console = require('z_sharp_std').console;

local IDENTIFIER = '^[a-zA-Z_$][a-zA-Z0-9_$]*';

z_sharp.lexer.create_chain('function')
	:capture(
		z_sharp.lexer.Rule.Single(
			z_sharp.lexer.SingleConfig.new({
				pattern = 'function',
				name = nil,
				required = true,
			})
		)
	)
	:trim()
	:logic(
		function (_)
			console.log(_);

			return true;
		end
	)
	:done()
;