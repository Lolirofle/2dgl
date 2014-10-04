#![feature(globs)]
#![feature(tuple_indexing)]

extern crate gl;
extern crate glfw;

use lolirofle::player::Player as Player;
use lolirofle::game::Game as Game;
use lolirofle::gameloop::Updatable as Updatable;
use lolirofle::gameloop::Renderable as Renderable;
use lolirofle::physics::WithPhysics as WithPhysics;
use lolirofle::physics::Existence as Existence;
use lolirofle::tdpg::window;
mod lolirofle;

struct TdpgGame{
	player: Player
}
impl Game for TdpgGame{
	fn init(&mut self){}
	fn close(&mut self){}
}
impl Updatable for TdpgGame{
	fn update(&mut self,delta_time: uint){
		self.player.update(delta_time);
	}
}
impl Renderable for TdpgGame{
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

	window::run();
}

//https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html
//http://smallcultfollowing.com/babysteps/blog/2013/10/24/single-inheritance/
