# Pixel Processing With Rust

Positioning remote artifacts in AR involves a lot of computation between animation frames. You need some pixel data from every frame, and a bunch of other data and secrets. This repo shows you how to obtain raw pixel data from each frame.

[A blog article about this work can be found here](https://dev.to/fallenstedt/using-rust-and-webassembly-to-process-pixels-from-a-video-feed-4hhg)

## Setup

1. Clone this repo and run `yarn` inside the `web` dir. 
2. `yarn start` will build rust and the web example
