use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::{Duration, Instant};
use std::fs::{self, OpenOptions, DirEntry};
use clap::Parser;
use log::{info, warn, error, debug};
use chrono::Utc;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
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

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
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
            body: vec![Position { x: GRID_WIDTH / 2, y: GRID_HEIGHT / 2 }],
            direction: Direction::Right,
            growing: false,
        }
    }

    fn update(&mut self) {
        debug!("Snake update: body length = {}, direction = {:?}, growing = {}", 
               self.body.len(), self.direction, self.growing);
        
        // Get current head position
        let head = self.body[0];
        debug!("Current head position: ({}, {})", head.x, head.y);
        
        // Calculate new head position
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y.saturating_sub(1) },
            Direction::Down => Position { x: head.x, y: (head.y + 1).min(GRID_HEIGHT - 1) },
            Direction::Left => Position { x: head.x.saturating_sub(1), y: head.y },
            Direction::Right => Position { x: (head.x + 1).min(GRID_WIDTH - 1), y: head.y },
        };

        debug!("New head position: ({}, {})", new_head.x, new_head.y);

        // Add new head
        self.body.insert(0, new_head);
        
        // Remove tail if not growing
        if !self.growing {
            let removed = self.body.pop();
            debug!("Removed tail segment: {:?}", removed);
        } else {
            self.growing = false;
            debug!("Snake growing, keeping tail");
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        debug!("Direction change attempt: {:?} -> {:?}", self.direction, new_direction);
        
        // Prevent the snake from going backwards into itself
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) | 
            (Direction::Down, Direction::Up) | 
            (Direction::Left, Direction::Right) | 
            (Direction::Right, Direction::Left) => {
                warn!("Invalid direction change blocked: {:?} -> {:?}", self.direction, new_direction);
                return;
            },
            _ => {
                self.direction = new_direction;
                debug!("Direction changed to: {:?}", self.direction);
            }
        }
    }

    fn grow(&mut self) {
        debug!("Snake growing triggered");
        self.growing = true;
    }

    fn check_collision(&self) -> bool {
        let head = self.body[0];
        debug!("Checking collision for head at: ({}, {})", head.x, head.y);
        
        // Check if head hits the walls
        if head.x == 0 || head.x >= GRID_WIDTH - 1 || head.y == 0 || head.y >= GRID_HEIGHT - 1 {
            warn!("Wall collision detected at: ({}, {})", head.x, head.y);
            return true;
        }

        // Check if head hits the body
        for (i, segment) in self.body[1..].iter().enumerate() {
            if head.x == segment.x && head.y == segment.y {
                warn!("Self collision detected at: ({}, {}) with body segment {}", head.x, head.y, i + 1);
                return true;
            }
        }

        debug!("No collision detected");
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

    fn spawn(&mut self, snake: &Snake) {
        debug!("Spawning new food, avoiding snake body of length: {}", snake.body.len());
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        
        loop {
            let x = rng.gen_range(1..GRID_WIDTH - 1);
            let y = rng.gen_range(1..GRID_HEIGHT - 1);
            attempts += 1;
            
            debug!("Food spawn attempt {}: trying position ({}, {})", attempts, x, y);
            
            // Make sure food doesn't spawn on snake
            let mut valid = true;
            for (i, segment) in snake.body.iter().enumerate() {
                if segment.x == x && segment.y == y {
                    debug!("Food spawn blocked by snake segment {} at ({}, {})", i, x, y);
                    valid = false;
                    break;
                }
            }
            
            if valid {
                self.position = Position { x, y };
                info!("Food spawned at: ({}, {}) after {} attempts", x, y, attempts);
                break;
            }
        }
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
        info!("Initializing new game with refresh rate: {}ms", refresh_rate);
        
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
                info!("Food eaten at: ({}, {}), score: {} -> {}", 
                      head.x, head.y, self.score, self.score + 10);
                self.snake.grow();
                self.score += 10;
                self.food.spawn(&self.snake);
            }

            // Check for collisions
            if self.snake.check_collision() {
                error!("Game over! Final score: {}", self.score);
                self.game_over = true;
            }
        }
    }

    fn handle_input(&mut self, window: &Window) {
        if self.game_over {
            if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
                info!("Game restart requested");
                self.restart();
            }
            return;
        }

        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            debug!("Up key pressed");
            self.snake.change_direction(Direction::Up);
        }
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            debug!("Down key pressed");
            self.snake.change_direction(Direction::Down);
        }
        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) {
            debug!("Left key pressed");
            self.snake.change_direction(Direction::Left);
        }
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) {
            debug!("Right key pressed");
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
                if x < GRID_SIZE || x >= WINDOW_WIDTH - GRID_SIZE || 
                   y < GRID_SIZE || y >= WINDOW_HEIGHT - GRID_SIZE {
                    buffer[y * WINDOW_WIDTH + x] = 0xFFFFFF; // White
                }
            }
        }
    }

    fn restart(&mut self) {
        info!("Restarting game");
        self.snake = Snake::new();
        self.food = Food::new();
        self.food.spawn(&self.snake);
        self.score = 0;
        self.game_over = false;
        self.last_update = Instant::now();
    }
}

fn setup_logging() -> String {
    let log_dir = if cfg!(target_os = "windows") {
        "C:\\var\\rusty-snake-logs"
    } else {
        "/var/rusty-snake-logs"
    };
    
    // Create log directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(log_dir) {
        eprintln!("Warning: Could not create log directory {}: {}", log_dir, e);
        // Fallback to temp directory
        let fallback_dir = if cfg!(target_os = "windows") {
            "C:\\temp\\rusty-snake-logs"
        } else {
            "/tmp/rusty-snake-logs"
        };
        if let Err(e) = fs::create_dir_all(fallback_dir) {
            eprintln!("Warning: Could not create fallback log directory {}: {}", fallback_dir, e);
            return if cfg!(target_os = "windows") {
                "C:\\temp\\rusty-snake.log".to_string()
            } else {
                "/tmp/rusty-snake.log".to_string()
            };
        }
        // Clean up old logs in fallback directory
        cleanup_old_logs(fallback_dir);
        return format!("{}/rusty-snake-{}.log", fallback_dir, Utc::now().format("%Y%m%d-%H%M%S"));
    }
    
    // Clean up old logs (keep only last 5)
    cleanup_old_logs(log_dir);
    
    // Generate unique log filename with timestamp
    let log_path = format!("{}/rusty-snake-{}.log", log_dir, Utc::now().format("%Y%m%d-%H%M%S"));
    log_path
}

fn cleanup_old_logs(log_dir: &str) {
    println!("Cleaning up old logs in: {}", log_dir);
    
    if let Ok(entries) = fs::read_dir(log_dir) {
        let mut log_files: Vec<DirEntry> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "log") &&
                entry.file_name().to_string_lossy().starts_with("rusty-snake-")
            })
            .collect();
        
        println!("Found {} log files", log_files.len());
        
        // Sort by modification time (newest first)
        log_files.sort_by(|a, b| {
            let a_time = a.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(std::time::UNIX_EPOCH);
            let b_time = b.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(std::time::UNIX_EPOCH);
            b_time.cmp(&a_time)
        });
        
        // Remove files beyond the 5 most recent
        let files_to_remove = log_files.len().saturating_sub(5);
        println!("Need to remove {} old log files", files_to_remove);
        
        for (i, file) in log_files.iter().enumerate() {
            if i >= 5 {
                println!("Removing old log file: {:?}", file.path());
                if let Err(e) = fs::remove_file(file.path()) {
                    eprintln!("Warning: Could not remove old log file {:?}: {}", file.path(), e);
                } else {
                    println!("Successfully removed: {:?}", file.path());
                }
            }
        }
    } else {
        eprintln!("Warning: Could not read log directory: {}", log_dir);
    }
}

fn main() {
    let cli = Cli::parse();
    
    // Setup logging with unique filename and cleanup
    let log_path = setup_logging();
    
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .expect("Failed to open log file")
        )))
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    info!("Starting Rusty Snake with refresh rate: {}ms", cli.refresh_rate);
    println!("Starting Rusty Snake with refresh rate: {}ms", cli.refresh_rate);
    println!("Logging to: {}", log_path);
    println!("Use arrow keys to move, R to restart, ESC to exit");
    
    let mut window = Window::new(
        &format!("Rusty Snake - Refresh Rate: {}ms", cli.refresh_rate),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        error!("Unable to create window: {}", e);
        panic!("Unable to create window: {}", e);
    });

    let mut game = Game::new(cli.refresh_rate);
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    info!("Game loop starting");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.handle_input(&window);
        game.update();
        game.render(&mut buffer);

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|e| {
                error!("Window update failed: {}", e);
            });
    }
    
    info!("Game loop ended");
    println!("\n🎮 Game session ended!");
    println!("📁 Game logs saved to: {}", log_path);
    println!("💡 You can view the logs with: cat \"{}\"", log_path);
}
