#![feature(tuple_indexing)]

use all::player::Player as Player;
use all::game::Game as Game;
use all::gameobj::Updatable as Updatable;
use all::physics::WithPhysics as WithPhysics;
use all::physics::Existence as Existence;
mod all;

struct TdpgGame{
	player: Player
}
impl Game for TdpgGame{
	fn init(&mut self){}
	fn close(&mut self){}
	fn update(&mut self,delta_time: uint){
		self.player.update(delta_time);
	}
	fn render(&self){}	
}

fn main(){
	let mut game = TdpgGame{
		player: Player::new()
	};
	for i in range(0u,10){
		println!("{}: At {} with mass {}",i,game.player.get_position(),game.player.get_mass());
		game.update(1);
	}
}

//https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html
//http://smallcultfollowing.com/babysteps/blog/2013/10/24/single-inheritance/
