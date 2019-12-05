use std::fs;
use std::io;
use std::time::Duration;

use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion};

fn target_01(c: &mut Criterion) {
    let day01 = fs::read_to_string("data/day1").unwrap();
    c.bench_function("day_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day01.as_bytes());
            aoc2019::day01::run(reader).unwrap();
        })
    });
}

fn target_02(c: &mut Criterion) {
    let day02 = fs::read_to_string("data/day2").unwrap();
    c.bench_function("day_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day02.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}

fn target_03(c: &mut Criterion) {
    let day03 = fs::read_to_string("data/day3").unwrap();
    c.bench_function("day_03", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day03.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}

criterion_group! {
    name = group;
    config = Criterion::default().warm_up_time(Duration::from_secs(5));
    targets = target_01, target_02,target_03
}

criterion_main!(group);