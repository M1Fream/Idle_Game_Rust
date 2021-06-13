use rand::Rng;
use crate::game_types;

pub struct Tile {
	my_type: TileType,
	unlocked: bool,
}

pub struct WorldMap {
	tiles : [[Tile; WorldMap::MAP_SIZE]; WorldMap::MAP_SIZE]
}

pub enum TileType {
	Empty,
	Terrain(TerrainType),
	Building(game_types::Building),
}

#[derive(FromPrimitive)]
pub enum TerrainType {
	Forest = 0,
	Lake = 1,
	Rock = 2,
	//If you update this, UPDATE NUM_TERRAINS TOO!
}

impl TerrainType{
	pub const NUM_TERRAINS : usize = 3; //This must be the nunmber of enum variants of TerrainType. Otherwise, it will create errors.

	pub fn rand() -> TerrainType {
		return num::FromPrimitive::from_usize(rand::thread_rng().gen_range(0..TerrainType::NUM_TERRAINS));
	}
}

impl WorldMap {
	pub const MAP_SIZE : usize = 255;

	/*pub fn new() -> WorldMap{
		//Add worldgen here
		return 
	}*/
}