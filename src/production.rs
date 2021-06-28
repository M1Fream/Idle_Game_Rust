use crate::game_types;
use arrayvec::ArrayVec;
use std::ops;
use crate::world;

pub fn test() -> usize {
	return 2;
}

trait production_obj {
	fn calc_prod(&self, g: &game_types::Game, i: usize, j: usize) -> game_types::Resources;
}

trait Bonus {
	fn calc_ratio(&self, g: &game_types::Game, tile : world::Tile) -> f64;
}
/*
trait StandardBonuses<'a> {
	fn get_edge_bonuses(&self) -> &Vec<&'a Bonus>;
	fn get_corner_bonuses(&self) -> &Vec<&'a Bonus>;
	fn get_upgrade_bonuses(&self) -> &Vec<&'a Bonus>;
	fn calc_bonuses(&self, g: &game_types::Game, i: usize, j: usize) -> f64 {
		let mut mult = 1.0;
		for b in self.get_edge_bonuses() {
			if i!=0 {mult *= b.calc_ratio(g, i-1, j)}
			if j!=0 {mult *= b.calc_ratio(g, i, j-1)}
			if i!=world::WorldMap::MAP_SIZE {mult *= b.calc_ratio(g, i+1, j)}
			if j!=world::WorldMap::MAP_SIZE {mult *= b.calc_ratio(g, i, j+1)}
		}
		for b in self.get_corner_bonuses() {
			if i!=0 && j!=0 {mult *= b.calc_ratio(g, i-1, j-1)}
			if i!=0 && j!=world::WorldMap::MAP_SIZE {mult *= b.calc_ratio(g, i-1, j+1)}
			if i!=world::WorldMap::MAP_SIZE && j!= 0 {mult *= b.calc_ratio(g, i+1, j-1)}
			if i!=world::WorldMap::MAP_SIZE && j!=world::WorldMap::MAP_SIZE {mult *= b.calc_ratio(g, i+1, j+1)}
		}
		for b in self.get_upgrade_bonuses() {
			mult *= b.calc_ratio(g, i, j);
		}
		return mult;
	}
}
*/
trait HasBonus<'a> {
	fn get_bonuses(&self) -> &Vec<&'a Bonus>;
	fn calc_bonuses(&self, g: &game_types::Game, tile: world::Tile) -> f64 {
		let mut ret = 1.0;
		for bonus in self.get_bonuses() {
			ret *= bonus.calc_ratio(g, tile);
		}
		return ret;
	}
}

struct ResetBonus {
	pub tier: usize,
}

impl Bonus for ResetBonus {
	fn calc_ratio(&self, g: &game_types::Game, tile: world::Tile) -> f64 {
		return 1.0; //TODO: Figure out reset currency and math
	}
}

struct UpgradeBonus {
	pub ratio: f64,
	pub upgrade: String,
}

impl Bonus for UpgradeBonus {
	fn calc_ratio(&self, g: &game_types::Game, tile: world::Tile) -> f64 {
		if g.upgrades[&self.upgrade].unlocked {
			return self.ratio;
		} else {
			return 1.0;
		}
	}
}

struct TileBonus<'a> {
	pub ratio: f64,
	pub tile: world::TileType,
	bonuses: Vec<&'a Bonus>,
	get_tiles: Fn(world::WorldMap, world::Tile) -> Vec<world::Tile>,
}


impl<'a> Bonus for TileBonus<'a> {
	fn calc_ratio(&self, g: &game_types::Game, tile: world::Tile) -> f64 {
		let t = self.tile;
		if matches!(tile.my_type, t) {
			return self.ratio * self.calc_bonuses(g, tile);
		} else {
			return 1.0;
		}
	}
}

impl<'a> HasBonus<'a> for TileBonus<'a> {
	fn get_bonuses(&self) -> &Vec<&'a Bonus> {
		return &self.bonuses;
	}
}

struct basic_production<'a> {
	pub base: game_types::Resources,
//	pub edge_bonuses: Vec<&'a Bonus>,
//	pub corner_bonuses: Vec<&'a Bonus>,
//	pub upgrade_bonuses: Vec<&'a Bonus>,
	pub bonuses: Vec<&'a Bonus>,
	pub tier: usize,
}

impl<'a> HasBonus<'a> for basic_production<'a> {
	fn get_bonuses(&self) -> &Vec<&'a Bonus> {
		return &self.bonuses;
	}
}

impl<'a> basic_production<'a> {
	pub fn new(res: game_types::Resources, bonuses: Vec<&'a Bonus>, tier : usize) -> basic_production<'a> {
		//return basic_production {base: res, edge_bonuses: edge, corner_bonuses: corner, upgrade_bonuses: upgrade, tier: tier};
		return basic_production {base: res, bonuses: bonuses, tier: tier};
	}
}
/*
impl<'a> StandardBonuses<'a> for basic_production<'a> {
	fn get_edge_bonuses(&self) -> &Vec<&'a Bonus> {return &self.edge_bonuses}
	fn get_corner_bonuses(&self) -> &Vec<&'a Bonus> {return &self.corner_bonuses}
	fn get_upgrade_bonuses(&self) -> &Vec<&'a Bonus> {return &self.upgrade_bonuses}
}*/

impl<'a> production_obj for basic_production<'a> {
	fn calc_prod(&self, g: &game_types::Game, i: usize, j:usize) -> game_types::Resources {
		let mut mult : f64 = 1.0;
		return self.base * mult;
	}
}

impl ops::Index<game_types::BuildingType> for [Box<dyn production_obj>] {
	type Output = Box<dyn production_obj>;
	
	fn index<'a>(self: &'a [Box<dyn production_obj>], idx: game_types::BuildingType) -> &'a  Box<dyn production_obj> {
		return &self[idx as usize];
	}
}

impl ops::IndexMut<game_types::BuildingType> for [Box<dyn production_obj>] {
//	type Output = Box<dyn production_obj>;
	
	fn index_mut<'a>(self: &'a mut [Box<dyn production_obj>], idx: game_types::BuildingType) -> &'a mut Box<dyn production_obj> {
		return &mut self[idx as usize];
	}
}

fn init_prod_objs() ->  ArrayVec<Box<dyn production_obj>, { game_types::Building::NUM_BUILDINGS }> {
//	let mut ret_vec : ArrayVec<Box<dyn production_obj>> = ArrayVec::with_capacity(game_types::Resources::NUM_RESOURCES);
	let mut ret_vec : ArrayVec<Box<dyn production_obj>, { game_types::Building::NUM_BUILDINGS }> = ArrayVec::<_, {game_types::Building::NUM_BUILDINGS}>::new();
	for _ in 0..game_types::Building::NUM_BUILDINGS {
		ret_vec.push(Box::new(basic_production::new(game_types::Resources::new(Vec::new()), Vec::new(), 1)));
	} //we initialize everything
	ret_vec[game_types::BuildingType::Farm] = Box::new(basic_production::new(game_types::Resources::new(vec![(game_types::ResourceType::Food, 5.0)]), Vec::new(), 1));
	return ret_vec;
}

pub fn calc_production(g: game_types::Game, prod_objs: ArrayVec<Box<dyn production_obj>, { game_types::Building::NUM_BUILDINGS }>) -> game_types::Resources {
	let mut ret = game_types::Resources {_res :[0.0; game_types::Resources::NUM_RESOURCES]};
	for i in 0..world::WorldMap::MAP_SIZE {
		for j in 0..world::WorldMap::MAP_SIZE {
			let cur_tile : world::Tile = g.world_map.tiles[i][j];
			match cur_tile.my_type {
				world::TileType::Building(b) => ret = ret + prod_objs[b.my_type].calc_prod(&g, i, j),
				_ => (),
			}
		}
	}
	return ret
}
