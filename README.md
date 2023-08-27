# screeps-arena-starter-rust

Starter Rust AI for [Screeps: Arena][screeps-arena], the JavaScript-based programming
strategy game.

This uses the [`screeps-arena-game-api`] bindings from the [rustyscreeps] organization.

It's also recommended to use [`cargo-screeps`] for building the code.

The documentation is currently a bit sparse. API docs which list functions one
can use are located at https://docs.rs/screeps-arena-game-api/.

Almost all crates on https://crates.io/ are usable (only things which interact with OS
apis are broken).

Quickstart:

```sh
# cli dependencies:
# TEMPORARY - get the arena branch of the cargo-screeps tool, which supports arena
git clone https://github.com/rustyscreeps/cargo-screeps.git
cd cargo-screeps
git checkout arena
cargo install --path .
cd ..
# TEMPORARY once arena is merged, go back to simply:
cargo install cargo-screeps

# clone:
git clone https://github.com/rustyscreeps/screeps-arena-starter-rust.git
cd screeps-arena-starter-rust

# configure with the path to your arena code directory:
cp example-screeps.toml screeps.toml
nano screeps.toml

# build tool:
cargo screeps --help
# compile the module
cargo screeps build
# compile plus deploy code to the game directory for the Swamp & Spawn arena
cargo screeps deploy -m swamp
```

[screeps-arena]: https://store.steampowered.com/app/1137320/Screeps_Arena/
[`cargo-screeps`]: https://github.com/rustyscreeps/cargo-screeps/
[`screeps-arena-game-api`]: https://github.com/rustyscreeps/screeps-arena-game-api/
[rustyscreeps]: https://github.com/rustyscreeps/
