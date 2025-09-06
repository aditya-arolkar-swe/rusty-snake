# Rusty Snake

A classic Snake game implementation in Rust using the `minifb` library for graphics and window management.

## Features

- Classic Snake gameplay
- Smooth movement with arrow key controls
- Food spawning and collision detection
- Score tracking
- Game over detection with restart functionality
- Clean, pixelated graphics

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

## Technical Details

- **Language**: Rust
- **Graphics Library**: minifb
- **Window Size**: 800x600 pixels
- **Grid Size**: 20x20 pixel cells
- **Game Grid**: 40x30 cells
- **Update Rate**: ~6.67 FPS (150ms per frame)

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd rusty-snake

# Run the game
cargo run

# Build for release
cargo build --release
```

## Dependencies

- `minifb`: For window management and graphics rendering
- `rand`: For random food placement

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

The game loop handles input processing, game state updates, and rendering in sequence, providing smooth gameplay at a consistent frame rate.
