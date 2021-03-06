//! This crate contains client-only components of Playform.

#![deny(missing_docs)]
#![deny(warnings)]
#![allow(deprecated)]

#![feature(duration)]
#![feature(main)]
#![feature(scoped)]
#![feature(test)]
#![feature(unboxed_closures)]
#![feature(range_inclusive)]
#![feature(cmp_partial)]
#![feature(ptr_as_ref)]

extern crate cgmath;
extern crate common;
extern crate env_logger;
extern crate gl;
#[macro_use]
extern crate log;
extern crate libc;
extern crate num;
extern crate sdl2;
extern crate sdl2_sys;
extern crate stopwatch;
extern crate test;
extern crate time;
extern crate yaglw;

mod camera;
mod client;
mod fontloader;
mod hud;
mod light;
mod load_terrain;
mod main;
mod mob_buffers;
mod player_buffers;
mod process_event;
mod render;
mod server_update;
mod shaders;
mod terrain_buffers;
mod ttf;
mod update_thread;
mod vertex;
mod view;
mod view_thread;
mod view_update;
