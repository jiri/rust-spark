#![feature(core)]

use std::f32::MAX_VALUE as f32_max;
use std::num::Float;

/// `graph` generates a string representation of the values as a sparkline.
///
/// # Arguments
///
/// * `values` - The values to graph.
///
/// # Example
///
/// ```rust
/// let sparkline = spark::graph(&[1.0, 5.0, 22.0, 13.0, 53.0]);
/// assert_eq!(sparkline, "▁▁▃▂█");
/// ```
pub fn graph(values: &[f32]) -> String {
    let ticks = "▁▂▃▄▅▆▇█";

    /* XXX: This doesn't feel like idiomatic Rust */
    let mut min: f32 = f32_max;
    let mut max: f32 = 0.0;

    for &i in values.iter() {
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

    values.iter()
        .cloned()
        .map(|n| (n - min) * ratio)
        .map(|n| n.floor() as usize)
        .filter_map(|n| ticks.chars().nth(n))
        .collect()
}
