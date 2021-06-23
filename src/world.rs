use rand::Rng;
use crate::game_types;

#[derive(Copy, Clone)]
pub struct Tile {
	pub my_type: TileType,
	unlocked: bool,
	x: usize,
	y: usize,
}

pub fn get_edge_neighbors(map: WorldMap, tile: Tile) -> Vec<Tile> {
	return Vec::new();
}

pub struct WorldMap {
//	pub tiles : [[Tile; WorldMap::MAP_SIZE]; WorldMap::MAP_SIZE]
	pub tiles : Vec<Vec<Tile>>,
}

#[derive(Copy, Clone)]
pub enum TileType {
	Empty,
	Terrain(TerrainType),
	Building(game_types::Building),
}

#[derive(FromPrimitive, Copy, Clone)]
pub enum TerrainType {
	Forest = 0,
	Lake = 1,
	Rock = 2,
	//If you update this, UPDATE NUM_TERRAINS TOO!
}

impl TerrainType{
	pub const NUM_TERRAINS : usize = 3; //This must be the nunmber of enum variants of TerrainType. Otherwise, it will create errors.

	pub fn rand() -> TerrainType {
		let rand_terrain = num::FromPrimitive::from_usize(rand::thread_rng().gen_range(0..TerrainType::NUM_TERRAINS));
		return rand_terrain.unwrap();
	}
}

impl WorldMap {
	pub const MAP_SIZE : usize = 63;

	/*pub fn new() -> WorldMap{
		//Add worldgen here
		return 
	}*/
	pub fn get_edge_neighbors(self, tile: Tile) -> Vec<Tile> {
		let mut ret : Vec<Tile> = Vec::new();
		let x = tile.x;
		let y = tile.y;
		if x!=0 {ret.push(self.tiles[x-1][y])}
		if y!=0 {ret.push(self.tiles[x][y-1])}
		if x!=WorldMap::MAP_SIZE {ret.push(self.tiles[x+1][y])}
		if y!=WorldMap::MAP_SIZE {ret.push(self.tiles[x][y+1])}
		return ret;
	}
	
	pub fn get_corner_neighbors(self, tile: Tile) -> Vec<Tile> {
		let mut ret : Vec<Tile> = Vec::new();
		let x = tile.x;
		let y = tile.y;
		if x!=0 && y!= 0 {ret.push(self.tiles[x-1][y-1])}
		if x!=0 && y!= WorldMap::MAP_SIZE {ret.push(self.tiles[x-1][y+1])}
		if x!=WorldMap::MAP_SIZE && y!= 0 {ret.push(self.tiles[x+1][y-1])}
		if x!=WorldMap::MAP_SIZE && y!=WorldMap::MAP_SIZE {ret.push(self.tiles[x+1][y+1])}
		return ret;
	}
}

impl Default for TileType {
	fn default() -> Self {
		return TileType::Empty;
	}
}

impl Default for Tile {
	fn default() -> Self {
		Self {
			my_type: TileType::default(),
			unlocked: false,
			x: 0,
			y: 0,
		}
	}
}

impl Default for WorldMap {
	fn default() -> Self {
		let mut tiles : Vec<Vec<Tile>> = Vec::new();
		for i in 0..WorldMap::MAP_SIZE {
			tiles.push(Vec::new());
			for j in 0..WorldMap::MAP_SIZE {
				tiles[i].push(Tile {my_type: TileType::default(), unlocked: false, x: i, y:j});
			}
		}
		Self {tiles: tiles}
	}
}