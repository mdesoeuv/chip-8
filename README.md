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

### References

- https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908
- instruction set : https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
- technical reference : https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference 
