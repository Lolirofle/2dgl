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
extern crate rustrt;
extern crate time;

pub mod data;
pub mod game_handler;
pub mod game;
mod gl;
pub mod graphics;
