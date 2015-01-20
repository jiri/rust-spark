use std::f32::MAX_VALUE as f32_max;
use std::num::Float;

pub fn graph(args: &[f32]) -> String {
    let ticks = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    let mut sparkline = String::with_capacity(args.len());

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

    let ratio = (ticks.len() - 1) as f32 / (max - min);

    for &i in args.iter() {
        sparkline.push_str(ticks[((i - min) * ratio).floor() as usize]);
    }

    sparkline
}
