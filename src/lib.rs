#![allow(unstable)]

use std::f32::MAX_VALUE as f32_max;
use std::num::Float;

pub fn graph(args: &[f32]) -> String {
    let ticks = "▁▂▃▄▅▆▇█";

    /* XXX: This doesn't feel like idiomatic Rust */
    let mut min: f32 = f32_max;
    let mut max: f32 = 0.0;

    for &i in args.iter() {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }

    let ratio = if max == min {
        1.0
    } else {
        (ticks.chars().count() - 1) as f32 / (max - min)
    };

    args.iter()
        .cloned()
        .map(|n| (n - min) * ratio)
        .map(|n| n.floor() as usize)
        .filter_map(|n| ticks.chars().nth(n))
        .collect()
}
