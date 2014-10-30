#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate glfw;

use lolirofle::tdpg::TdpgGame;
use lolirofle::game_handler::GameHandler as GameHandlerTrait;
//use lolirofle::game_handler::multithreaded::GameHandler;
use lolirofle::game_handler::singlethreaded::GameHandler;
mod lolirofle;

fn main(){
	GameHandler::<TdpgGame>.run();
}

//https://mail.mozilla.org/pipermail/rust-dev/2013-November/006714.html
//http://smallcultfollowing.com/babysteps/blog/2013/10/24/single-inheritance/
