//! This crate contains server-only components of Playform.

#![deny(missing_docs)]
#![deny(warnings)]

#![feature(core)]
#![feature(collections)]
#![feature(main)]
#![feature(std_misc)]
#![feature(test)]
#![feature(unboxed_closures)]
#![feature(unsafe_destructor)]

extern crate capnp;
#[macro_use]
extern crate "capnpc-macros" as capnpc_macros;
extern crate cgmath;
extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nanomsg;
extern crate noise;
extern crate opencl;
extern crate rand;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;
extern crate time;

mod client_recv_thread;
mod in_progress_terrain;
mod init_mobs;
mod main;
mod mob;
mod octree;
mod opencl_context;
mod physics;
mod player;
mod server;
mod sun;
mod terrain;
mod update_gaia;
mod update_world;
