use std::sync::Arc;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;
use parallel_computing_lab3::lab3::*;

const ARRAY_SIZE: usize = 1700000;
// const ARRAY_SIZE: usize = 100;

pub fn bench_functions(c: &mut Criterion) {
    let mut array: Vec<u64> = vec![0; ARRAY_SIZE];
    let mut rng = rand::thread_rng();
    for i in 0..array.len() {
        array[i] = rng.gen_range(0..10000);
    }
    // let arr_arc: Arc<&[u64]> = array.into();
    let mut group = c.benchmark_group("Sum of elements > 10");
    group.bench_with_input(
        BenchmarkId::new("Sequential", 0), &array,
        |b, i| b.iter(||{
            proceed_seq(i);
        }),
    );
    for p in (1..5).map(|n| {2_i32.pow(n)}) {
        group.bench_with_input(
            BenchmarkId::new("Parallel Crossbeam", p), &array,
            |b, i| b.iter(||{
                proceed_par_crossbeam(i, p as usize);
            }),
        );
    }



    for p in (1..5).map(|n| {2_i32.pow(n)}) {
        group.bench_with_input(
            BenchmarkId::new("Parallel Atomic fetch_add", p), &array,
            |b, i| b.iter(||{
                proceed_par_arc(array.clone().into(), p as usize);
            }),
        );
    }

    for p in (1..5).map(|n| {2_i32.pow(n)}) {
        group.bench_with_input(
            BenchmarkId::new("Parallel Atomic compare_exchange", p), &array,
            |b, i| b.iter(||{
                proceed_par_arc_comp_ex(array.clone().into(), p as usize);
            }),
        );
    }

    group.bench_with_input(
        BenchmarkId::new("Parallel Rayon", 0), &array,
        |b, i| b.iter(||{
            proceed_par_rayon(i);
        }),
    );



}


criterion_group!(benches, bench_functions);
criterion_main!(benches);