--!strict
--[=[
	Function declatation.
--]=]

local z_sharp = require('@z_sharp');

local lexer = z_sharp.lexer;

local patterns = {
	IDENTIFIER = '[a-zA-Z_$][a-zA-Z0-9_$]+',
}; -- require('patterns.luau'); -- Load local files.

local Whitespace = require('@src/chains/whitespace.luau');

local Function = {
	chain = (function ()
		local chain = lexer.Chain.new('function');
		
		chain:capture(
			lexer.Rule.Single({
				name = nil,
				pattern = 'function',
				required = true,
			})
		);

		chain:capture(
			Whitespace.child(true)
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
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\(',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		-- TODO: Function parameters.

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\)',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\:',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = patterns.IDENTIFIER, -- TODO: Replace with type.
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(false)
		);

		-- TODO: Use section instead of `{` and `}`.
		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\{',
					required = true,
				})
			)
		);

		-- TODO: Function body.

		chain:capture(
			Whitespace.child(false)
		);

		chain:capture(
			lexer.Rule.Single(
				lexer.SingleConfig.new({
					name = nil,
					pattern = '\\}',
					required = true,
				})
			)
		);
		
		chain:capture(
			lexer.Rule.Logic(
				lexer.LogicConfig.new({
					func = function (results): boolean
						z_sharp.console.log(results.identifier.data);
						return true;
					end,
				})
			)
		);

		chain:done();

		return chain;
	end)(),
};

return Function;