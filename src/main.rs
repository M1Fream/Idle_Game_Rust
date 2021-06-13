mod production;

use std::ops;
//use std::string;

//const NUM_RESOURCES: usize = 2;

#[derive(Copy, Clone)]
struct Resources {
	_res: [f64; Resources::NUM_RESOURCES],
}

impl ops::Index<ResourceType> for Resources {
	type Output = f64;
	
	fn index(self: &Resources, idx: ResourceType) -> &f64 {
		return &self._res[idx as usize];
	}
}

struct Game {
	paused: bool,
	tiles: [[Tile; 255]; 255],
	resources: Resources,
	production: Resources,
	techs: [Tech; Tech::NUM_TECHS],
//	upgrades: [Upgrade: Upgrade::NUM_UPGRADES],
}

struct Tech {
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

struct Tile {
	my_type: TileType,
	unlocked: bool,
}

//#[derive(Eq, Hash)]
enum ResourceType {
	Wood = 0,
	Food = 1,
}

enum TileType {
	Empty,
	Terrain(TerrainType),
	Building(Building),
}

enum TerrainType {
	Forest,
	Lake,
	Rock,
}

struct Building {
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
	const NUM_RESOURCES : usize = 2;
	fn can_buy(self, cost: Resources) -> bool {
		let mut i: usize = 0;
		while(i<Resources::NUM_RESOURCES) {
			if(cost._res[i] > self._res[i]) {
				return false;
			}
			i += 1;
		}
		return true;
	}
}



fn main() {
    println!("Hello, world!");
	/*let res1 = Resources {wood: 2.0, food: 4.0};
	let res2 = Resources {wood: 3.0, food: 5.0};
	let res3 = res1 + res2;
	println!("{}, {}", res3.wood, (res3 * 1.5).food);*/
	let res4 = Resources{_res: [2.0, 4.0]};
	let res5 = Resources{_res: [3.0, 5.0]};
	let res6 = res4 - res5;
	println!("{}, {}", res6._res[0], (res6 * 1.5)._res[ResourceType::Food as usize]);
	println!("{}, {}", res6._res[0], (res6 * 2.5)[ResourceType::Food]);
	println!("{}", Resources::NUM_RESOURCES);
//	let myTile = TileType::Farm;
//	println!("{}", get_hover_str(myTile));
	println!("{}", production::test());
}
