[![Build Status](https://travis-ci.org/bfops/playform.svg?branch=master)](https://travis-ci.org/bfops/playform)

## Introduction

Playform aspires to be an open-world sandbox game written in Rust, taking
inspiration from [Voxel Farm](http://procworld.blogspot.com/) and Minecraft.

It's currently.. well, very much a WIP. As
[michaelwu's C++ bindgen fork](https://github.com/michaelwu/rust-bindgen/tree/sm-hacks)
becomes more and more capable, part of the plan is to start using seasoned C++
libraries like [bullet physics](https://github.com/bulletphysics/bullet3).

Help is great! PRs and [issues](https://github.com/bfops/playform/issues)
are appreciated.

Some (oudated) screenshots:

![screenshot 1](/../screenshots/screenshots/screenshot1.png?raw=true)
![screenshot 2](/../screenshots/screenshots/screenshot2.png?raw=true)

## Making it work

Make sure you have:

  * The **nightly build** of the Rust compiler and package manager, `cargo`.
  * OpenCL
  * `libpng`
  * `SDL2`
  * `SDL2_ttf`
  * `libnanomsg`

At any point, `--release` can be appended onto `cargo build` or `cargo run` for a slower
build, but a much more optimized result.

Run the Playform server using `cargo run` in the `server` folder. It takes one parameter:
the listen URL for the server. It defaults to running locally: `ipc:///tmp/server.ipc`.

The client can be run similarly with `cargo run` in the `client` folder. It takes two
parameters: the listen URL of the client and the listen URL of the server. They
both default to running locally (`ipc:///tmp/client.ipc` for the client URL).

**Some dependencies might not build**. Look for forks that are updated for
your `rustc`, and then point your `~/.cargo/config` at them.

If you find `playform` itself won't build on the latest `rustc`, please open an issue or file a PR!

## How to play

  * Move: WASD
  * Jump: Space
  * Look around: Mouse

One mob spawns that will play "tag" with you: tag it and it will chase you until it tags you back.

## If things don't work

If things are broken, like compile errors, problems getting it to start, crashes, etc.
please consider opening an issue! If you can take the time to do it in a non-optimized
build with `RUST_BACKTRACE=1` and `RUST_LOG=debug` set, it would be much appreciated.
