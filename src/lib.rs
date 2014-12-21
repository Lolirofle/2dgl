//! Library for 2D games

#![crate_name = "2dgl"]
#![crate_type = "lib"]

#![feature(globs)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate num;
extern crate time;

pub mod data;
pub mod game;
pub mod game_handler;
pub mod graphics;
