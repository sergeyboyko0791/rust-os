Implement an x86_64 OS in Rust following this guide: https://os.phil-opp.com/minimal-rust-kernel/

## Build an executable binary file

Build an executable binary file with a linked bootloader (compatible with BIOS).

```shell
cargo bootimage 
```

## Run in QEMU

1. Install QEMU (MacOS specific):

```shell
brew install qemu
```

2. Run QEMU emulator with the result binary file compiled via `cargo bootimage`:

```shell
cargo run
```

## Run tests

1. Install `xbuild`:

```shell
cargo install xbuild
```

2. Run unit and integration tests:

```shell
cargo xtest
```
