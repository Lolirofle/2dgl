#![crate_name = "2dgl"]
#![comment = "Personal library for building games"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![feature(tuple_indexing)]

extern crate core;
extern crate collections;
extern crate gl;
extern crate rustrt;
extern crate time;

pub mod data;
pub mod game_handler;
pub mod game;
pub mod graphics;
