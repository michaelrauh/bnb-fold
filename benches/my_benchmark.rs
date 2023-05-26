use std::fs::read_to_string;

use bnb_fold::{
    rule::{get_diagonals, get_impacted_phrase_locations, make_blank},
    solve::solve_for_dims,
    string_handlers::make_codec,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn full_benchmark(c: &mut Criterion) {
    c.bench_function("full_benchmark", |b| {
        b.iter(|| solve_for_dims(black_box(vec![2, 2])))
    });
}

fn test_blank(c: &mut Criterion) {
    c.bench_function("test_blank", |b| {
        b.iter(|| make_blank(black_box(&vec![2, 2])))
    });
}

// fn test_full(c: &mut Criterion) {
//     c.bench_function("test_full", |b| {
//         b.iter(|| {
//             full(black_box(&vec![
//                 Some("here".to_string()),
//                 Some("is".to_string()),
//                 Some("an".to_string()),
//                 Some("example".to_string()),
//             ]))
//         })
//     });
// }

fn test_impacted(c: &mut Criterion) {
    c.bench_function("test_impacted", |b| {
        b.iter(|| get_impacted_phrase_locations(&vec![2, 2]))
    });
}

// fn test_next_open(c: &mut Criterion) {
//     c.bench_function("test_next_open", |b| {
//         b.iter(|| {
//             next_open_position(&vec![
//                 Some("here".to_string()),
//                 Some("is".to_string()),
//                 Some("an".to_string()),
//                 None,
//             ])
//         })
//     });
// }

fn test_diagonals(c: &mut Criterion) {
    c.bench_function("test_diagonals", |b| b.iter(|| get_diagonals(&vec![2, 2])));
}

fn test_vocabulary(c: &mut Criterion) {
    let corpus = read_to_string("example.txt").unwrap();
    c.bench_function("test_vocabulary", |b| b.iter(|| make_codec(&corpus)));
}

// fn test_make_set(c: &mut Criterion) {
//     let corpus = read_to_string("example.txt").unwrap();
//     c.bench_function("test_make_set", |b| b.iter(|| corpus_to_set(&corpus, 100)));
// }

criterion_group!(benches, full_benchmark);
criterion_main!(benches);
