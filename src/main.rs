#![feature(tuple_indexing)]

use game::{TdpgGame,Player,Point2};
mod game;

fn main(){
	let mut game = TdpgGame{
		player: Player{
			position: Point2(0.0,50.0),
			velocity: Point2(0.0,0.0)
		}
	};
	for i in range(0u,5){
		println!("{}: At {} with mass {}",i,game.player.position,game.player.get_mass());
		game.update(1);
	}
}

//https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html
//http://smallcultfollowing.com/babysteps/blog/2013/10/24/single-inheritance/
