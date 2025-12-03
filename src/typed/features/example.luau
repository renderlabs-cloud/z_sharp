--- # Example

local Core = require('./core');

local Human: Core.Class<boolean, string> = Core.class () {
	constructor = function(self, alive: boolean, name: string)
		self.alive = alive;
		self.name = name;

		print('Human constructor');
		print('Alive:', alive);
		print('Name:', name);
	end,

	alive = nil,
	name = nil,

	kill = function(self)
		self.alive = false;
		
		print(self.name .. ' is dead :(');
	end,
};

local myHuman = Human(true, 'Bill');

getmetatable(Human).__call();

--[[local Bill: Core.Class<boolean> = Core.class (Human) {
	constructor = function(self, alive: boolean)
		self.prototype.super(true, 'Bill');
		print('Bill constructor');
	end,
};

local myBill = Bill(true);]]