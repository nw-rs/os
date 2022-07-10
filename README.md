# NW OS

An operating system for the Numworks n0110 calculator written in Rust.

Currently WIP does not actually do anything and currently requires an STLink
debugger to install because I do not have a working bootloader for external
flash.

## Setup

To setup the developement environment follow the instructions in
[`rustworks/SETUP.md`](https://github.com/nw-rs/)


## STLink

If you have an STLink debugger (I am using the STLink V3SET) you can flash or
debug using one of the following commands (note that this seems to write the 
data to internal flash which is not desireable as that only has 64KiB of space
avaliable):

### Flash

Specify the chip manually:

```zsh
cargo flash --chip=stm32f730V8Tx
```

Let `cargo-make` specify the chip for you:

```zsh
cargo make flash
```

### Debug

Using `cargo-embed` (recommended):

```zsh
cargo embed
```

Using `probe-rs`:

```zsh
cargo run
```

## DFU flash

Complete setup, install a DFU bootloader capable of writing to external flash
(currently there are none available,
[`nw-rs/bootloader`](https://github.com/nw-rs/bootloader) is still WIP and
doesn't work properly), plug in your calculator and put it into DFU mode (press
6 and the reset button on the back at the same time), then run the following
command:

```zsh
cargo make dfu
```

