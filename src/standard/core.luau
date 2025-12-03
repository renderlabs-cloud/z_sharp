--!strict
--[[
-- # Currently this file is for testing.
---- In the future the file(s) will be the standard.
---- Anyways, thanks for checking this project out!
----
---- TODO: Compile this BEFORE loading into Z#. I'm not sure how to do this yet.
---- TODO: Split into multiple files.
----
---- @Author Brendan Lucas
----
--]]

local z_sharp = require('@z_sharp');

local patterns = {
	IDENTIFIER = '[a-zA-Z_$][a-zA-Z0-9_$]+',
}; -- require('patterns.luau'); -- Load local files.

local Whitespace = {
	chain = (function ()
		local chain = z_sharp.lexer.Chain.new('whitespace');

		chain:capture(
			z_sharp.lexer.Rule.Single(
				z_sharp.lexer.SingleConfig.new({
					name = nil,
					pattern = '\\s+',
					required = true,
				})
			)
		);

		chain:done();

		return chain;
	end)(),
};

function Whitespace.child(required: boolean): ChildConfig
	return z_sharp.lexer.Rule.Child(
		z_sharp.lexer.ChildConfig.new({
			name = nil,
			child = Whitespace.chain,
			required = required,
		})
	) :: ChildConfig;
end;

z_sharp.lexer.register_chain(Whitespace.chain);

local Function = {
	chain = (function ()
		local chain = z_sharp.lexer.Chain.new('function');
		
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
	end)()
	,	
};

z_sharp.lexer.register_chain(Function.chain);