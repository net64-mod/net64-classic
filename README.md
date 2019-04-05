# Net64 Classic

New implementation for Net64 in Rust.
It uses Mupen64Plus as built-in emulator.

## Project Status

**This project is in a very early development state.**

Currently all [Mupen64Plus APIs](https://mupen64plus.org/wiki/index.php?title=Mupen64Plus_v2.0_Core_API_v1.0) need to be implemented and called properly.
I did not yet manage to successfully load a SM64 ROM.

Mupen64Plus only supports 8MB RAM, so we either have to implement 16MB ourselves or we move back to old Net64 v1 patches.

## Project Goals

- cross platform (Windows, Linux, Mac, RetroArch, 32/64Bit)
- stability -> competitive for speedrunning
- remove client-emulator separation
