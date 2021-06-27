use crate::world;
use std::ops;

pub struct Game {
	pub paused: bool,
	pub world_map: world::WorldMap,
	pub resources: Resources,
	pub production: Resources,
//	pub techs: [Tech; Tech::NUM_TECHS],
//	upgrades: [Upgrade: Upgrade::NUM_UPGRADES],
}

impl Game {
	pub fn buy(&mut self, b: &buyable) -> bool {
		let c = b.get_cost();
		if self.resources.can_buy(c) {
			self.resources = self.resources - c;
			return true;
		} else {
			return false;
		}
	}
}

trait buyable {
	fn get_cost(&self) -> Resources;
}

#[derive(Copy, Clone)]
pub struct Resources {
	pub _res: [f64; Resources::NUM_RESOURCES],
}

impl ops::Index<ResourceType> for Resources {
	type Output = f64;
	
	fn index(self: &Resources, idx: ResourceType) -> &f64 {
		return &self._res[idx as usize];
	}
}

impl ops::IndexMut<ResourceType> for Resources {
	fn index_mut(self: &mut Resources, idx: ResourceType) -> &mut f64 {
		return &mut self._res[idx as usize];
	}
}

pub struct Tech {
	name: String,
	num: usize,
	cost: Resources,
	tier: usize,
	desc: String,
	unlocked: bool,
	unlock: Box<Fn(Game) -> bool>,
}

impl Tech {
	const NUM_TECHS : usize = 2;
}

#[derive(Copy, Clone)]
pub enum ResourceType {
	Wood = 0,
	Food = 1,
}

#[derive(Copy, Clone)]
pub struct Building {
	pub my_type: BuildingType,
	level: u8,
}

impl Building {
	pub const NUM_BUILDINGS : usize = 12;
}

#[derive(Copy, Clone)]
pub enum BuildingType {
	Farm = 0,
	Stable = 1,
	Water_well = 2,
	Furnace_simple = 3,
	Steam_engine = 4,
	Electric_motor = 5,
	Altar = 6,
	Shrine = 7,
	Temple = 8,
	Library = 9,
	Mana_well = 10,
	Occult_library = 11,
}
/*
fn get_hover_str(x: TileType) -> String {
	match x {
		TileType::Empty => String::from("An empty space"),
		TileType::Farm => String::from("A basic farm"),
		TileType::Tree => String::from("Literally a tree"),
	}
}
*/
impl ops::Add<Resources> for Resources {
	type Output = Resources;
	
	fn add(self, _rhs: Resources) -> Resources {
		let mut new_res: [f64; Resources::NUM_RESOURCES] = [0.0; Resources::NUM_RESOURCES];
		let mut i: usize = 0;
		while(i<Resources::NUM_RESOURCES) {
			new_res[i] = self._res[i] + _rhs._res[i];
			i += 1;
		}
		Resources {
			_res: new_res
		}
	}
}

impl ops::Sub<Resources> for Resources {
	type Output = Resources;
	
	fn sub(self, _rhs : Resources) -> Resources {
		self + (_rhs * -1.0)
	}
}

impl ops::Mul<f64> for Resources {
	type Output = Resources;
	
	fn mul(self: Resources, _rhs: f64) -> Resources {
		let mut new_res: [f64; Resources::NUM_RESOURCES] = [0.0; Resources::NUM_RESOURCES];
		let mut i: usize = 0;
		while(i<Resources::NUM_RESOURCES) {
			new_res[i] = self._res[i] * _rhs;
			i += 1;
		}
		Resources {
			_res: new_res
		}
	}
}

impl Resources {
	pub const NUM_RESOURCES : usize = 2;
	pub fn can_buy(self, cost: Resources) -> bool {
		let mut i: usize = 0;
		while(i<Resources::NUM_RESOURCES) {
			if(cost._res[i] > self._res[i]) {
				return false;
			}
			i += 1;
		}
		return true;
	}
	pub fn new(in_res: Vec<(ResourceType, f64)>) -> Resources {
		let mut res: [f64; Resources::NUM_RESOURCES] = [0.0; Resources::NUM_RESOURCES];
		for (r, ammount) in in_res.iter() {
			res[(*r) as usize] = *ammount;
		}
		return Resources {_res: res};
	}
}

impl Default for Resources {
	fn default() -> Self {
		Self {_res: [0.0; Resources::NUM_RESOURCES]}
	}
}

impl Default for Game {
	fn default() -> Self {
		Self {
			paused: false,
			world_map: world::WorldMap::default(),
			resources: Resources::default(),
			production: Resources::default(),
		}
	}
}