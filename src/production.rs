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

fn prod_nothing(g: game_types::Game, i: usize, j: usize) -> game_types::Resources {
	return game_types::Resources::new(Vec::new());
}

#[derive(Copy, Clone)]
struct Bonus {
	pub ratio: f64,
	pub tile: world::TileType,
}

struct basic_production {
	base: game_types::Resources,
	edge_bonuses: Vec<Bonus>,
	corner_bonuses: Vec<Bonus>,
	tier: usize,
}

impl basic_production {
	pub fn new(res: game_types::Resources, edge: Vec<Bonus>, corner: Vec<Bonus>, tier : usize) -> basic_production {
		return basic_production {base: res, edge_bonuses: edge, corner_bonuses: corner, tier: tier};
	}
}

impl production_obj for basic_production {
	fn calc_prod(&self, g: &game_types::Game, i: usize, j:usize) -> game_types::Resources {
		let mut ret = self.base;
		let mut mult : f64 = 1.0;
		for b in &self.edge_bonuses {
			let cnt = count_edge(g, i, j, b.tile);
			mult *= 1.0 + b.ratio * (cnt as f64);
		}
		for b in &self.corner_bonuses {
			let cnt = count_corner(g, i, j, b.tile);
			mult *= 1.0 + b.ratio * (cnt as f64);
		}
		return self.base * mult;
	}
}

fn count_edge(g: &game_types::Game, i: usize, j:usize, t: world::TileType) -> usize {
	let mut ret = 0;
	if i!=0 && matches!(g.world_map.tiles[i-1][j].my_type, t) { ret += 1; }
	if j!=0 && matches!(g.world_map.tiles[i][j-1].my_type, t) { ret += 1; }
	if i!=world::WorldMap::MAP_SIZE && matches!(g.world_map.tiles[i+1][j].my_type, t) { ret += 1; }
	if j!=world::WorldMap::MAP_SIZE && matches!(g.world_map.tiles[i][j+1].my_type, t) { ret += 1; }
	return ret;
}

fn count_corner(g: &game_types::Game, i: usize, j:usize, t: world::TileType) -> usize {
	let mut ret = 0;
	if i!=0 && j!=0 && matches!(g.world_map.tiles[i-1][j-1].my_type, t) { ret += 1; }
	if i!=0 && j!=world::WorldMap::MAP_SIZE && matches!(g.world_map.tiles[i-1][j+1].my_type, t) { ret += 1; }
	if i!=world::WorldMap::MAP_SIZE && j != 0 && matches!(g.world_map.tiles[i+1][j-1].my_type, t) { ret += 1; }
	if i!=world::WorldMap::MAP_SIZE && j!=world::WorldMap::MAP_SIZE && matches!(g.world_map.tiles[i+1][j+1].my_type, t) { ret += 1; }
	return ret;
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
		ret_vec.push(Box::new(basic_production::new(game_types::Resources::new(Vec::new()), Vec::new(), Vec::new(), 1)));
	} //we initialize everything
	ret_vec[game_types::BuildingType::Farm] = Box::new(basic_production::new(game_types::Resources::new(vec![(game_types::ResourceType::Food, 5.0)]), Vec::new(), Vec::new(), 1));
	return ret_vec;
}

/*
fn prod_basic_factory(res: &'static game_types::Resources) -> (Box<Fn(game_types::Game, usize, usize) -> game_types::Resources>) {
	return Box::new(|g: game_types::Game, i: usize, j: usize| -> game_types::Resources {return *res});
}

static prod_funcs : [Box<Fn(game_types::Game, usize, usize) -> game_types::Resources>; game_types::Resources::NUM_RESOURCES] = [Box::new(prod_nothing); game_types::Resources::NUM_RESOURCES];
*/
//let prod_funcs = [prod_nothing; game_types::Resources::NUM_RESOURCES];


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