<p align="center">
  <img width="400" height="400" alt="Untitled design (1)" src="https://github.com/user-attachments/assets/86fa48e0-57a9-4c3c-a663-cde32df2f3fa" />
</p>

# Rusty Snake
A classic Snake game implementation in Rust using the `minifb` library for graphics and window management.

## Features

- Classic Snake gameplay
- Smooth movement with arrow key controls
- Food spawning and collision detection
- Score tracking with visual display
- Game over detection with restart functionality
- Clean, pixelated graphics
- **Customizable refresh rate via CLI arguments**

## Controls

- **Arrow Keys**: Move the snake (Up, Down, Left, Right)
- **R**: Restart the game when game over
- **ESC**: Exit the game

## How to Play

1. Run the game with `cargo run`
2. Use arrow keys to control the snake
3. Eat the red food to grow and increase your score
4. Avoid hitting the walls or your own body
5. Try to get the highest score possible!

## Game Rules

- The snake starts moving to the right
- Each food eaten increases your score by 10 points
- The snake grows by one segment each time it eats food
- The game ends if the snake hits a wall or itself
- Press 'R' to restart after game over

## Command Line Options

The game supports customizable refresh rates for different difficulty levels:

```bash
# Default speed (150ms refresh rate)
cargo run

# Faster game (50ms refresh rate) - More challenging!
cargo run -- --refresh-rate 50

# Slower game (300ms refresh rate) - Easier for beginners
cargo run -- --refresh-rate 300

# Show help
cargo run -- --help

# Show version
cargo run -- --version
```

### Refresh Rate Guide

- **50ms**: Very fast - Expert level
- **100ms**: Fast - Advanced level  
- **150ms**: Default - Normal level
- **200ms**: Slow - Beginner level
- **300ms**: Very slow - Easy mode

## Technical Details

- **Language**: Rust
- **Graphics Library**: minifb
- **CLI Parsing**: clap
- **Window Size**: 800x600 pixels
- **Grid Size**: 20x20 pixel cells
- **Game Grid**: 40x30 cells
- **Default Update Rate**: ~6.67 FPS (150ms per frame)

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd rusty-snake

# Run with default settings
cargo run

# Run with custom refresh rate
cargo run -- --refresh-rate 100

# Build for release
cargo build --release

# Run release build
./target/release/rusty-snake --refresh-rate 75
```

## Dependencies

- `minifb`: For window management and graphics rendering
- `rand`: For random food placement
- `clap`: For command-line argument parsing

## Project Structure

```
rusty-snake/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main game implementation
└── README.md           # This file
```

## Game Architecture

The game is structured with several key components:

- **Position**: Represents grid coordinates
- **Direction**: Enum for snake movement direction
- **Snake**: Manages snake body, movement, and growth
- **Food**: Handles food placement and collision detection
- **Game**: Main game state and logic coordination
- **Cli**: Command-line argument parsing structure

The game loop handles input processing, game state updates, and rendering in sequence, providing smooth gameplay at a configurable frame rate.

## Examples

```bash
# Play at expert speed
cargo run -- --refresh-rate 50

# Play at beginner speed  
cargo run -- --refresh-rate 250

# Get help
cargo run -- --help
```
