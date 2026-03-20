#  Quantum Terminal — A Reality-Bending CLI

> “A terminal where commands are non-deterministic, files behave unpredictably, and reality itself is inconsistent.”

Quantum Terminal is an interactive experience built in Rust that simulates a shell where the very fabric of the virtual environment is unstable. Every command you run influences the "Reality Engine," increasing entropy and causing the system to behave in increasingly unsettling ways.

##  Features

- **Reality Engine**: A central brain tracking entropy, user habits, and timeline drift.
- **Schrödinger Files**: A virtual filesystem where files can emerge, disappear, or change content based on the current reality state.
- **Lying `ls`**: Returns deceptive results, fake files, and occasionally "impossible" filenames.
- **Haunted `ps`**: Monitors processes that shouldn't exist, like `ghost_daemon` or `you.exe`.
- **Time-Dependent `open`**: Access files to find past logs or future records that haven't been written yet.
- **Glitch Mode**: Visual distortions, flickering text, and ANSI character corruption that increase with entropy.

##  Tech Stack

- **Rust**: High-performance systems language.
- **Crossterm**: Terminal control and event handling.
- **Ratatui**: Terminal UI rendering.
- **Tokio**: Asynchronous runtime for concurrent reality shifts.
- **Rand**: Weighted randomness (controlled chaos).
- **Serde**: Persistent state management.

## Usage

1. **Clone the repository.**
2. **Build and Run**:
   ```bash
   cargo run
   ```
3. **Commands to try**:
   - `ls`: See what (currently) exists.
   - `cat <file>`: Read a file... if it lets you.
   - `ps`: Check for anomalies in the process list.
   - `open <file>`: Peek into other timelines.
   - `exit`: Try to escape.

## The Reality Engine

The engine monitors your every move. Repeating the same commands or staying in the terminal too long increases **Entropy**. As entropy rises:
- Files become less stable.
- Text begins to glitch and corrupt.
- The timeline drifts into the past or future.
- The system may begin to address you directly.

---

*“Something is wrong… but designed that way.”*
