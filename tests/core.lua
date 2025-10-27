local z_sharp = require('z_sharp');

local IDENTIFIER = '^[a-zA-Z_$][a-zA-Z0-9_$]*';

local END_OF_LINE = '\\;';

local DUAL_OPERATORS = {
	'\\+',
	'\\-',
	'\\/',
	'\\*',
	--- ...
};

z_sharp.create_capture_zone('function')
	.capture({
		Single = {
			pattern = 'function'
		}
	})
	.trim()
	.capture({
		Single = {
			pattern = IDENTIFIER,
			name = 'name',
		}
	})
	.trim()
	.capture({
		Single = {
			pattern = '\\(',
		}
	})
	.trim()
	.capture({
		Repeat = {
			rules = {
				[1] = {
					Single = {
						pattern = IDENTIFIER,
						name = 'name',
					},
				},
				[2] = {
					Single = {
						pattern = '\\:'
					},
				},
				-- TODO: Add type support
				[3] = {
					Single = {
						pattern = 'byte',
					},
				},
			},
			seperator = {
				Single = {
					pattern = '\\,',
				}
			},
			name = 'parameters',
		}
	})
	.trim()
	.capture({
		Single = {
			pattern = '\\)',
		}
	})
	.trim()
	.capture({
		Single = {
			pattern = '->',
		}
	})
	.trim()
	-- TODO: Add type support
	.capture({
		Single = {
			pattern = 'byte',
			name = 'type',
		}
	})
	.trim()
	.capture({
		Single = {
			pattern = '\\{',
		}
	})
	.trim()
	-- 	.block(0)
	.capture({
		Single = {
			pattern = '\\}',
		}
	})
	.trim()
	-- .logic(function (_)
		
	-- end)
	.done({

	})
;

console.log(z_sharp.__);