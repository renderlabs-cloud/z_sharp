--!strict
--[[
-- # Semantics.
--]]

local z_sharp = require('@z_sharp');

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

return Whitespace;