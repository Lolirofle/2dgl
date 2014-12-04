//! Library for 2D games

#![crate_name = "2dgl"]
#![crate_type = "lib"]

#![feature(globs)]
#![feature(if_let)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate num;
extern crate rustrt;
extern crate time;

pub mod data;
pub mod game;
pub mod game_handler;
pub mod graphics;
