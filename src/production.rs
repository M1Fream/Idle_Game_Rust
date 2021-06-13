use crate::game_types;

pub fn test() -> usize {
	return 2;
}

fn prod_nothing(g: game_types::Game, i: usize, j: usize) -> game_types::Resources {
	return game_types::Resources::new(Vec::new());
}
/*
fn prod_basic_factory(res: &'static game_types::Resources) -> (Box<Fn(game_types::Game, usize, usize) -> game_types::Resources>) {
	return Box::new(|g: game_types::Game, i: usize, j: usize| -> game_types::Resources {return *res});
}

static prod_funcs : [Box<Fn(game_types::Game, usize, usize) -> game_types::Resources>; game_types::Resources::NUM_RESOURCES] = [Box::new(prod_nothing); game_types::Resources::NUM_RESOURCES];
*/
//let prod_funcs = [prod_nothing; game_types::Resources::NUM_RESOURCES];

/*
pub fn calc_production(g: game_types::Game, step: f64) -> game_types::Resources {
	let mut i: usize = 0;
	let mut j: usize = 0;
	while(i<game_types::Game::MAP_SIZE) {
		while(j<game_types::Game::MAP_SIZE) {
			
}*/