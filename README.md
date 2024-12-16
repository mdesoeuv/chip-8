## Chip-8 Emulator

Simple Chip-8 emulator written in Rust

### Usage

#### Build Project

```shell
cargo build --release
```

#### Run

```shell
./target/release/chip-8
```

or build and run release at the same time : `cargo run -r`

### Log

Run the executable with the environment variable `RUST_LOG` set to a log level among:

- trace
- debug
- info
- warning
- error

example:

```sh
$ RUST_LOG="debug" cargo run
```

But you will get the logs from every carte that uses `log` crate.

To get a more specific log you can set log filters per crate,
Here our crate name is `chip_8` and **not** `chip-8`, which means we need to run:

```bash
$ RUST_LOG="chip_8=debug" cargo run
$ RUST_LOG=chip_8::machine,chip_8::machine::screen=off cargo run -r -- programs/7-beep.ch8 --debug
```

### Nix

On linux x86-64 this project provides a nix flake which provides the following:

```bash
# Enter development environment
$ nix develop
$ cargo run #... Edit, build, run & test project here
$ exit # Exit the environment shell like any shell

# Format the whole project
$ nix fmt # Rust, Nix, Markdown, YAML & TOML

# Build the packaged binary (not portable)
$ nix build
$ ./result/bin/chip-8 # Result here

# Or build & run it in one go
$ nix run -- programs/1-chip8-logo

# Or even it without cloning the repository
$ nix run github:mdesoeuv/chip-8
```

### References

- https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908
- instruction set : https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
- technical reference : https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
