use clap::Parser;
use minifb::{Key, Window, WindowOptions};
use rand::seq::IndexedRandom;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: usize = 1280;
const WINDOW_HEIGHT: usize = 720;
const GRID_SIZE: usize = 20;
const GRID_WIDTH: usize = WINDOW_WIDTH / GRID_SIZE;
const GRID_HEIGHT: usize = WINDOW_HEIGHT / GRID_SIZE;

#[derive(Parser)]
#[command(name = "rusty-snake")]
#[command(about = "A classic Snake game implementation in Rust")]
#[command(version)]
struct Cli {
    /// Refresh rate in milliseconds (lower = faster game)
    #[arg(long, default_value = "150")]
    refresh_rate: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<Position>,
    direction: Direction,
    growing: bool,
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: vec![Position {
                x: GRID_WIDTH / 2,
                y: GRID_HEIGHT / 2,
            }],
            direction: Direction::Right,
            growing: false,
        }
    }

    fn update(&mut self) {
        // Get current head position
        let head = self.body[0];

        // Calculate new head position
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Position {
                x: head.x,
                y: (head.y + 1).min(GRID_HEIGHT - 1),
            },
            Direction::Left => Position {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Position {
                x: (head.x + 1).min(GRID_WIDTH - 1),
                y: head.y,
            },
        };

        // Add new head
        self.body.insert(0, new_head);

        // Remove tail if not growing
        if !self.growing {
            self.body.pop();
        } else {
            self.growing = false;
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        // Prevent the snake from going backwards into itself
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => return,
            _ => self.direction = new_direction,
        }
    }

    fn grow(&mut self) {
        self.growing = true;
    }

    fn check_collision(&self) -> bool {
        let head = self.body[0];

        // Check if head hits the walls
        if head.x == 0 || head.x >= GRID_WIDTH - 1 || head.y == 0 || head.y >= GRID_HEIGHT - 1 {
            return true;
        }

        // Check if head hits the body
        for segment in &self.body[1..] {
            if head.x == segment.x && head.y == segment.y {
                return true;
            }
        }

        false
    }
}

struct Food {
    position: Position,
}

impl Food {
    fn new() -> Self {
        Food {
            position: Position { x: 0, y: 0 },
        }
    }

    /// Spawns food at a random position that doesn't overlap with the snake's body.
    /// Returns true if food was successfully spawned, false if no valid positions remain (game won).
    fn spawn(&mut self, snake: &Snake) -> bool {
        let valid_positions = self.get_valid_spawn_positions(snake);
        
        match valid_positions.choose(&mut rand::rng()) {
            Some(&position) => {
                self.position = position;
                true
            }
            None => {
                println!("Game Won! No more valid spawn positions.");
                false
            }
        }
    }

    /// Returns all valid positions where food can spawn (not occupied by snake or walls).
    fn get_valid_spawn_positions(&self, snake: &Snake) -> Vec<Position> {
        let mut valid_positions = Vec::new();
        
        // Create a set of snake body positions for O(1) lookup
        let snake_positions: std::collections::HashSet<Position> = 
            snake.body.iter().copied().collect();
        
        // Check each position in the playable area (excluding walls)
        for x in 1..GRID_WIDTH - 1 {
            for y in 1..GRID_HEIGHT - 1 {
                let position = Position { x, y };
                
                // Only add positions that aren't occupied by the snake
                if !snake_positions.contains(&position) {
                    valid_positions.push(position);
                }
            }
        }
        
        valid_positions
    }
}

struct Game {
    snake: Snake,
    food: Food,
    score: u32,
    game_over: bool,
    last_update: Instant,
    refresh_rate: Duration,
}

impl Game {
    fn new(refresh_rate: u64) -> Self {
        let mut game = Game {
            snake: Snake::new(),
            food: Food::new(),
            score: 0,
            game_over: false,
            last_update: Instant::now(),
            refresh_rate: Duration::from_millis(refresh_rate),
        };
        game.food.spawn(&game.snake);
        game
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        if self.last_update.elapsed() >= self.refresh_rate {
            self.snake.update();
            self.last_update = Instant::now();

            // Check if snake ate food
            let head = self.snake.body[0];
            if head.x == self.food.position.x && head.y == self.food.position.y {
                self.snake.grow();
                self.score += 10;
                self.food.spawn(&self.snake);
            }

            // Check for collisions
            if self.snake.check_collision() {
                self.game_over = true;
            }
        }
    }

    fn handle_input(&mut self, window: &Window) {
        if self.game_over {
            if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
                self.restart();
            }
            return;
        }

        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            self.snake.change_direction(Direction::Up);
        }
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            self.snake.change_direction(Direction::Down);
        }
        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) {
            self.snake.change_direction(Direction::Left);
        }
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) {
            self.snake.change_direction(Direction::Right);
        }
    }

    fn render(&self, buffer: &mut [u32]) {
        // Clear buffer (black)
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000; // Black
        }

        // Draw snake (green)
        for segment in &self.snake.body {
            let start_x = segment.x * GRID_SIZE;
            let start_y = segment.y * GRID_SIZE;
            for y in start_y..start_y + GRID_SIZE {
                for x in start_x..start_x + GRID_SIZE {
                    if y < WINDOW_HEIGHT && x < WINDOW_WIDTH {
                        buffer[y * WINDOW_WIDTH + x] = 0x00FF00; // Green
                    }
                }
            }
        }

        // Draw food (red)
        let start_x = self.food.position.x * GRID_SIZE;
        let start_y = self.food.position.y * GRID_SIZE;
        for y in start_y..start_y + GRID_SIZE {
            for x in start_x..start_x + GRID_SIZE {
                if y < WINDOW_HEIGHT && x < WINDOW_WIDTH {
                    buffer[y * WINDOW_WIDTH + x] = 0xFF0000; // Red
                }
            }
        }

        // Draw border (white)
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                if x < GRID_SIZE
                    || x >= WINDOW_WIDTH - GRID_SIZE
                    || y < GRID_SIZE
                    || y >= WINDOW_HEIGHT - GRID_SIZE
                {
                    buffer[y * WINDOW_WIDTH + x] = 0xFFFFFF; // White
                }
            }
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
        self.food.spawn(&self.snake);
        self.score = 0;
        self.game_over = false;
        self.last_update = Instant::now();
    }
}

fn main() {
    let cli = Cli::parse();

    println!(
        "Starting Rusty Snake with refresh rate: {}ms",
        cli.refresh_rate
    );
    println!("Use arrow keys to move, R to restart, ESC to exit");

    let mut window = Window::new(
        &format!("Rusty Snake - Refresh Rate: {}ms", cli.refresh_rate),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    let mut game = Game::new(cli.refresh_rate);
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.handle_input(&window);
        game.update();
        game.render(&mut buffer);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
