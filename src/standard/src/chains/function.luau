--!strict
--[[
-- # Function.
--]]

local z_sharp = require('@z_sharp');

local patterns = {
	IDENTIFIER = '[a-zA-Z_$][a-zA-Z0-9_$]+',
}; -- require('patterns.luau'); -- Load local files.

local Whitespace = require('@self/chains/whitespace.luau');

local Function = {
	chain = (function ()
		local chain = z_sharp.lexer.Chain.new('function');

		z_sharp.console.log(require('@self/chains/whitespace.luau'));
		
		chain:capture(
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
					name = nil,
					pattern = 'function',
					required = true,
				})
			)
		);

		chain:capture(
			Whitespace.child(true)
		);

		chain:capture(
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
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
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
					name = nil,
					pattern = '\\}',
					required = true,
				})
			)
		);
		
		chain:capture(
			z_sharp.lexer.Rule.Logic(
				z_sharp.lexer.LogicConfig.new({
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