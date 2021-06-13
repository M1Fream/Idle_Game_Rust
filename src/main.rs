mod production;
mod game_types;
mod world;
use game_types::Resources;
use game_types::ResourceType;

extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    println!("Hello, world!");
	/*let res1 = Resources {wood: 2.0, food: 4.0};
	let res2 = Resources {wood: 3.0, food: 5.0};
	let res3 = res1 + res2;
	println!("{}, {}", res3.wood, (res3 * 1.5).food);*/
//	let res4 = Resources{_res: [2.0, 4.0]};
//	let res5 = Resources{_res: [3.0, 5.0]};
//	let res6 = res4 - res5;
//	println!("{}, {}", res6._res[0], (res6 * 1.5)._res[ResourceType::Food as usize]);
//	println!("{}, {}", res6._res[0], (res6 * 2.5)[ResourceType::Food]);
	println!("{}", Resources::NUM_RESOURCES);
//	let myTile = TileType::Farm;
//	println!("{}", get_hover_str(myTile));
//	println!("{}", production::test());
}
