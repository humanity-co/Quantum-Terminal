# Quantum Terminal

A specialized Rust-based terminal shell simulating a terminal filesystem where files mutate state based on timeline drift and glitching memory engines.

## Technical Architecture
* **Glitch Simulator:** Evaluates entropy layers against memory states to mutate target parameters.
* **File System Layer:** Virtual memory-backed directory storage tracking file lifecycle states.
* **Shell Parsing:** Custom lexical parser translating inputs into executable process routines.
* **Visual Layer:** Terminal UI managing frame-buffer flushes and ANSI color maps.

## Repository Layout
* `/src/reality` - Core state transition engine, timelines, randomness calculators, and glitch drivers.
* `/src/shell` - Input parser, command executor, and history tracking.
* `/src/filesystem` - Virtual directory structures, unstable file hooks, and reality state files.
* `/src/ui` - Viewport renderers, effect managers, and layouts.
* `/src/commands` - Process logic (`cat`, `ls`, `ps`, `open`).

## Setup and Installation
1. Build the binary using Cargo:
   ```bash
   cargo build --release
   ```
2. Launch the terminal shell:
   ```bash
   cargo run
   ```

## License
Proprietary. All rights reserved.
