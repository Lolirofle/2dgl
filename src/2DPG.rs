#![feature(globs)]
#![feature(tuple_indexing)]

extern crate gl;
extern crate glfw;

use lolirofle::player::Player as Player;
use lolirofle::game::Game as Game;
use lolirofle::gameobj::Updatable as Updatable;
use lolirofle::physics::WithPhysics as WithPhysics;
use lolirofle::physics::Existence as Existence;
mod lolirofle;

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
