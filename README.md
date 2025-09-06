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
- **Comprehensive debug logging system with automatic file management**

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

## Debug Logging System

The game includes a comprehensive debugging system that automatically logs detailed game state information to files.

### Features
- **Automatic logging** - No flags needed, always active
- **Timestamped logs** with full game state snapshots
- **Organized file storage** in dedicated directories
- **Unique filenames** with timestamps to prevent conflicts
- **Automatic cleanup** - maintains only the 5 most recent log files
- **Cross-platform** support for Windows and Unix systems

### Log File Locations
- **Primary location**: `/var/rusty-snake-logs/` (Linux/macOS) or `C:\var\rusty-snake-logs\` (Windows)
- **Fallback location**: `/tmp/rusty-snake-logs/` (Linux/macOS) or `C:\temp\rusty-snake-logs\` (Windows)

### Log File Naming
- **Format**: `rusty-snake-YYYYMMDD-HHMMSS.log`
- **Example**: `rusty-snake-20250906-062636.log`
- **Unique per session** - no filename conflicts

### Automatic Cleanup
- **Keeps only 5 most recent log files**
- **Removes oldest files automatically** when more than 5 exist
- **Sorts by modification time** (newest first)
- **Runs on every game start** to maintain the limit

### Logged Events
- Game initialization and startup
- Snake movement and direction changes
- Food spawning attempts and success
- Collision detection (wall and self-collision)
- Score changes when food is eaten
- Game over conditions
- Game restarts
- Input handling (key presses)
- Window and rendering events

### Log Format
Each log entry includes timestamp, log level, and detailed information:
```
[2025-09-06T06:26:37Z INFO  rusty_snake] Starting Rusty Snake with refresh rate: 100ms
[2025-09-06T06:26:37Z INFO  rusty_snake] Initializing new game with refresh rate: 100ms
[2025-09-06T06:26:37Z DEBUG rusty_snake] Spawning new food, avoiding snake body of length: 1
[2025-09-06T06:26:37Z INFO  rusty_snake] Food spawned at: (30, 18) after 1 attempts
[2025-09-06T06:26:37Z DEBUG rusty_snake] Snake update: body length = 1, direction = Right, growing = false
[2025-09-06T06:26:37Z DEBUG rusty_snake] Current head position: (20, 15)
[2025-09-06T06:26:37Z DEBUG rusty_snake] New head position: (21, 15)
[2025-09-06T06:26:37Z WARN  rusty_snake] Wall collision detected at: (39, 15)
[2025-09-06T06:26:37Z ERROR rusty_snake] Game over! Final score: 0
```

### Game End Notification
When a game session ends, the terminal displays:
```
🎮 Game session ended!
📁 Game logs saved to: /tmp/rusty-snake-logs/rusty-snake-20250906-062636.log
💡 You can view the logs with: cat "/tmp/rusty-snake-logs/rusty-snake-20250906-062636.log"
```

## Technical Details

- **Language**: Rust
- **Graphics Library**: minifb
- **CLI Parsing**: clap
- **Logging**: log + env_logger
- **Serialization**: serde + serde_json
- **Time Handling**: chrono
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
- `log`: For logging framework
- `env_logger`: For logging implementation
- `serde`: For data serialization
- `serde_json`: For JSON serialization
- `chrono`: For timestamp handling

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

The game loop handles input processing, game state updates, and rendering in sequence, providing smooth gameplay at a configurable frame rate with comprehensive logging capabilities.

## Debugging Issues

If you encounter crashes or unexpected behavior:

1. **Check the terminal output** when the game ends for the log file path
2. **View the log file** using the provided command
3. **Look for patterns** in the logs around the crash point
4. **Check for error messages** in the log entries
5. **Report issues** with the relevant log entries

The debug logs provide complete game state snapshots that make it easy to identify the exact conditions that lead to bugs or crashes.

### Example Log Analysis
```bash
# View the most recent log
cat /tmp/rusty-snake-logs/rusty-snake-20250906-062636.log

# Search for specific events
grep "collision" /tmp/rusty-snake-logs/rusty-snake-20250906-062636.log
grep "ERROR" /tmp/rusty-snake-logs/rusty-snake-20250906-062636.log

# View all available logs
ls -la /tmp/rusty-snake-logs/
```

## Examples

```bash
# Play at expert speed
cargo run -- --refresh-rate 50

# Play at beginner speed  
cargo run -- --refresh-rate 250

# Get help
cargo run -- --help

# View logs after playing
cat /tmp/rusty-snake-logs/rusty-snake-20250906-062636.log
```

## Log Management

The logging system automatically manages log files:

- **Creates directories** if they don't exist
- **Generates unique filenames** with timestamps
- **Maintains exactly 5 log files** (removes older ones)
- **Provides clear feedback** about cleanup operations
- **Shows log file path** when game ends

This ensures you always have access to recent game logs for debugging while preventing disk space issues from accumulating old log files.
