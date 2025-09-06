use clap::Parser;
use minifb::{Key, Window, WindowOptions};

use rusty_snake::rusty_snake::{Game, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Parser)]
#[command(name = "rusty-snake")]
#[command(about = "A classic Snake game implementation in Rust")]
#[command(version)]
struct Cli {
    /// Refresh rate in milliseconds (lower = faster game)
    #[arg(long, default_value = "150")]
    refresh_rate: u64,
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
