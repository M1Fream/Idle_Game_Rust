use crate::game_types;

pub fn test() -> usize {
	return 2;
}

fn prod_nothing(g: game_types::Game, i: usize, j: usize) {
	return game_types::Resources {};
}

//let prod_funcs : [fn(game_types::Game, usize, usize); game_types::Resources::NUM_RESOURCES] = [prod_nothing; game_types::Resources::NUM_RESOURCES];

let prod_funcs = [prod_nothing; game_types::Resources::NUM_RESOURCES];

/*
pub fn calc_production(g: game_types::Game, step: f64) -> game_types::Resources {
	let mut i: usize = 0;
	let mut j: usize = 0;
	while(i<game_types::Game::MAP_SIZE) {
		while(j<game_types::Game::MAP_SIZE) {
			
}*/