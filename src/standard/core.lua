--[[
-- # Currently this file is for testing.
---- In the future the file(s) will be the standard.
---- Anyways, thanks for checking this project out!
---
-- TODO: Use tealr.
---
--]]

local z_sharp <const> = require('z_sharp');
local console <const> = require('z_sharp_std').console;

local IDENTIFIER <const> = '[a-zA-Z_$][a-zA-Z0-9_$]+';  -- TODO: Move to another file.

local Function <const> = {
	__chain__ = (function ()
			local chain = z_sharp.lexer.Chain.new('function');
			
			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = 'function',
						name = nil,
						required = true,
					})
				)
			);

			chain:trim(true);

			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = IDENTIFIER,
						name = 'identifier',
						required = true,
					})
				)
			);

			chain:trim(false);

			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = '\\(',
						name = nil,
						required = true,
					})
				)
			);

			chain:trim(false);

			-- TODO: Function parameters.
			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = '\\)',
						name = nil,
						required = true,
					})
				)
			);

			chain:trim(false);

			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = '\\:',
						name = nil,
						required = true,
					})
				)
			);

			chain:trim(false);

			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = IDENTIFIER, -- TODO: Replace with type.
						name = nil,
						required = true,
					})
				)
			);

			chain:trim(false);

			-- TODO: Use section instead of `{` and `}`.
			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = '\\{',
						name = nil,
						required = true,
					})
				)
			);

			-- TODO: Function body.
			chain:trim(false);

			chain:capture(
				z_sharp.lexer.Rule.Single(
					z_sharp.lexer.SingleConfig.new({
						pattern = '\\}',
						name = nil,
						required = true,
					})
				)
			);
			
			chain:logic(
				function (_)
					console.log(_);

					return true;
				end
			);

			chain:done();

			return chain;
		end)()
	,	
};

z_sharp.lexer.register_chain(Function.__chain__);