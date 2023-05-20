use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bnb_fold::{solve::{self, solve_for_dims}, rule::{make_blank, full, get_impacted_phrase_locations, next_open_position, get_diagonals}, string_handlers::{vocabulary, corpus_to_set}};


// fn full_benchmark(c: &mut Criterion) {
//     c.bench_function("full_benchmark", |b| b.iter(|| solve_for_dims(black_box(vec![2, 2]))));
// }

fn test_blank(c: &mut Criterion) {
    c.bench_function("test_blank", |b| b.iter(|| make_blank(black_box(&vec![2, 2]))));
}

fn test_full(c: &mut Criterion) {
    c.bench_function("test_full", |b| b.iter(|| full(black_box(&vec![Some("here".to_string()), Some("is".to_string()), Some("an".to_string()), Some("example".to_string())]))));
}

fn test_impacted(c: &mut Criterion) {
    c.bench_function("test_impacted", |b| b.iter(|| get_impacted_phrase_locations(&vec![2, 2])));
}

fn test_next_open(c: &mut Criterion) {
    c.bench_function("test_next_open", |b| b.iter(|| next_open_position(&vec![Some("here".to_string()), Some("is".to_string()), Some("an".to_string()), None])));
}

fn test_diagonals(c: &mut Criterion) {
    c.bench_function("test_diagonals", |b| b.iter(|| get_diagonals(&vec![2, 2])));
}

fn test_vocabulary(c: &mut Criterion) {
    let corpus = read_to_string("example.txt").unwrap();
    c.bench_function("test_vocabulary", |b| b.iter(|| vocabulary(&corpus)));
}

fn test_make_set(c: &mut Criterion) {
    let corpus = read_to_string("example.txt").unwrap();
    c.bench_function("test_make_set", |b| b.iter(|| corpus_to_set(&corpus)));
}

criterion_group!(benches, test_blank, test_full, test_impacted, test_next_open, test_diagonals, test_vocabulary, test_make_set);
criterion_main!(benches);