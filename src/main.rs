#[allow(unused_imports)]

mod days;

use chrono::{Datelike, Utc};
use std::time::Instant;
use aoc2024::utils;

fn main() {
    let day: u8 = Utc::now().date_naive().day() as u8;
    let day_solver = get_day_solver(day);
    let time = Instant::now();
    let (p1, p2) = day_solver(utils::get_input(day));
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    let elapsed_s = elapsed_ms / 1000.0;

    println!("\n=== Day {:02} ===", day);
    println!("  · Part 1: {}", p1);
    println!("  · Part 2: {}", p2);
    if elapsed_ms < 1000.0 {
        println!("  · Elapsed: {:.4} ms", elapsed_ms);
    } else {
        println!("  · Elapsed: {:.4} s", elapsed_s);
    }

}

fn get_day_solver(day: u8) -> fn(String) -> (usize,usize) {
    match day {
        1 => days::day1::solve,
        2 => days::day2::solve,
        3 => days::day3::solve,
        4 => days::day4::solve,
        5 => days::day5::solve,
        6 => days::day6::solve,
        7 => days::day7::solve,
        8 => days::day8::solve,
        9 => days::day9::solve,
        10 => days::day10::solve,
        11 => days::day11::solve,
        12 => days::day12::solve,
        13 => days::day13::solve,
        14 => days::day14::solve,
        15 => days::day15::solve,
        16 => days::day16::solve,
        17 => days::day17::solve,
        18 => days::day18::solve,
        19 => days::day19::solve,
        20 => days::day20::solve,
        21 => days::day21::solve,
        22 => days::day22::solve,
        23 => days::day23::solve,
        24 => days::day24::solve,
        25 => days::day25::solve,
        _ => unimplemented!("Day Not Found")
    }
}