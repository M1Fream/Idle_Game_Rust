use std::ops;

pub struct Game {
	paused: bool,
	tiles: [[Tile; Game::MAP_SIZE]; Game::MAP_SIZE],
	resources: Resources,
	production: Resources,
	techs: [Tech; Tech::NUM_TECHS],
//	upgrades: [Upgrade: Upgrade::NUM_UPGRADES],
}

impl Game {
	pub const MAP_SIZE : usize = 255;
}

#[derive(Copy, Clone)]
pub struct Resources {
	_res: [f64; Resources::NUM_RESOURCES],
}

impl ops::Index<ResourceType> for Resources {
	type Output = f64;
	
	fn index(self: &Resources, idx: ResourceType) -> &f64 {
		return &self._res[idx as usize];
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

pub struct Tile {
	my_type: TileType,
	unlocked: bool,
}

#[derive(Copy, Clone)]
pub enum ResourceType {
	Wood = 0,
	Food = 1,
}

pub enum TileType {
	Empty,
	Terrain(TerrainType),
	Building(Building),
}

pub enum TerrainType {
	Forest,
	Lake,
	Rock,
}

pub struct Building {
	my_type: BuildingType,
	level: u8,
}

enum BuildingType {
	Farm
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