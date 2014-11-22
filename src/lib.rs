//! Library for 2D games

#![crate_name = "2dgl"]
#![comment = "Personal library for building games"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![feature(phase)]
#![feature(if_let)]

extern crate core;
extern crate collections;
extern crate num;
extern crate rustrt;
extern crate time;

pub mod data;
pub mod game;
pub mod game_handler;
mod gl;
pub mod graphics;
