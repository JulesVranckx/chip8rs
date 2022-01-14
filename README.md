# CHIP-8 emulator 

/!\ WIP

## Introduction

This project has been written as a personnal project to learn both RUST and emulation. It is certainly not perfect. I tried to be as idiomatic as possible.
It is an chip-8 emulator, that mostly supports display for now.

## Dependencies
This project use several external crates and packages as dependency

### Crates

- rand, version > 0.8.0
- sdl2

### Packages

Run 
```
sudo apt-get install libsdl2-dev libsdl2-gfx-dev
```

to install needed libraries

### Usage

BUILD:
    cargo build --release

USAGE:
    chip8 [OPTIONS]

OPTIONS:
    -f, --file <FILE>    program to be executed
    -h, --help           Print help information
    -r, --raw            input file as raw
    -t, --text           input file as text
    -V, --version        Print version information

### Notes

I only developped the back end of the emulator.
I took all the display and sound drivers from https://github.com/starrhorne/chip8-rust

### Referencies

[Complete specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Fx0A)
[Quite the same spec, more readable with some time diagrams](http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf)
[Useful repo for drivers & other](https://github.com/starrhorne/chip8-rust)

### Personnal TODO

Implement drivers by myself
Add the keyboard handling
Fix the not responding window issu
Write documentation
Fix LD_F instruction

