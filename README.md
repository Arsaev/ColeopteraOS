# ColeopteraOS

**OS for RaspberryPi based on Rust using CSCI 320 course**

## Already available:

- Freestanding Rust binary code
  How to compile for ARM embedded system
  1. Add target
     `rustup target add thumbv7em-none-eabihf`
  2. Build for target
     `cargo build --target thumbv7em-none-eabihf`

Learning/Overview:

| Ressource                                     | URL                                          |
| --------------------------------------------- | -------------------------------------------- |
| CSCI 320                                      | http://ozark.hendrix.edu/~ferrer/courses/os/ |
| CSCI 320 (newer Version)                      | https://hendrix-cs.github.io/csci320/        |
| Writing an OS in Rust (mentioned by CSCI 320) | https://os.phil-opp.com/                     |

Example Code:

| Ressource                                     | URL                                                                                                                                                     |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bare Program using VGA Buf                    | https://github.com/gjf2a/bare_metal_1                                                                                                                   |
| Pacman(?) Clone using VGA Buf                 | https://github.com/gjf2a/ghost_hunter // https://github.com/gjf2a/ghost_hunter_core // Doc: https://hendrix-cs.github.io/csci320/projects/baremetalgame |
| Writing an OS in Rust (mentioned by CSCI 320) | https://github.com/phil-opp/blog_os                                                                                                                     |
| Another OS for RPI in Rust tutorial/repo      | https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials                                                                                          |

Useful Tools:

| Ressource      | URL                                |
| -------------- | ---------------------------------- |
| RasPi Emulator | https://github.com/jhhoward/Faux86 |
