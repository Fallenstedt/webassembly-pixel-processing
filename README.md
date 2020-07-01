# Pixel Processing With Rust

Positioning remote artifacts in AR involves a lot of computation between animation frames. To begin this, endeavor, you need some WebRTC and some pixel data. This repo shows you how to obtain raw pixel data from each frame.

[A blog article about this work can be found here](https://dev.to/fallenstedt/using-rust-and-webassembly-to-process-pixels-from-a-video-feed-4hhg)

## Setup

1. Clone this repo and run `yarn` inside the `web` dir. 
2. `yarn start` will build rust and the web example