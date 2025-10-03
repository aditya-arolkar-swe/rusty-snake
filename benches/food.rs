use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::{rng, seq::SliceRandom};
use rusty_snake::rusty_snake::{Food, Position, Snake, GRID_HEIGHT, GRID_WIDTH};

fn compare_short(c: &mut Criterion) {
    let snake = Snake::new();
    let mut food = Food::new();

    c.bench_function("spawn short snake", |b| {
        b.iter(|| food.spawn(black_box(&snake)))
    });
    c.bench_function("spawn_hash short snake", |b| {
        b.iter(|| food.spawn_hash(black_box(&snake)))
    });
}

fn compare_medium(c: &mut Criterion) {
    let snake = Snake::init(generate_positions(GRID_WIDTH * GRID_HEIGHT / 2));
    let mut food = Food::new();

    c.bench_function("spawn medium snake", |b| {
        b.iter(|| food.spawn(black_box(&snake)))
    });
    c.bench_function("spawn_hash medium snake", |b| {
        b.iter(|| food.spawn_hash(black_box(&snake)))
    });
}

fn compare_long(c: &mut Criterion) {
    let snake = Snake::init(generate_positions(GRID_WIDTH * GRID_HEIGHT - 1));
    let mut food = Food::new();

    c.bench_function("spawn long snake", |b| {
        b.iter(|| food.spawn(black_box(&snake)))
    });
    c.bench_function("spawn_hash long snake", |b| {
        b.iter(|| food.spawn_hash(black_box(&snake)))
    });
}

fn generate_positions(num: usize) -> Vec<Position> {
    // 1. Generate a flat list of all coordinates.
    let mut all_points: Vec<Position> = (0..GRID_WIDTH)
        .flat_map(|y| (0..GRID_HEIGHT).map(move |x| Position { x, y }))
        .collect();

    // Shuffle the entire list randomly.
    let mut rng = rand::rng();
    all_points.shuffle(&mut rng);

    // Take the first half of the shuffled list.
    all_points.into_iter().take(num).collect()
}

criterion_group!(benches, compare_short, compare_medium, compare_long);
criterion_main!(benches);
