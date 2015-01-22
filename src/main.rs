#![allow(unstable)]

extern crate spark;

use std::os;
use std::io::stdio::stdin_raw;

/* TODO: docs */
fn main() {
    /* XXX: This is pretty weird */
    let args = os::args();
    let mut args = args.tail().iter();

    if args.any(|s| s.contains("-v")) {
        println!("spark-rs v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.any(|s| s.contains("-h")) || args.count() == 0 {
        println!("Usage:\n\tspark [-hv] VALUE,...");
        return;
    }

    /* XXX: This is becoming a bit too much */
    let values: Vec<f32> = if stdin_raw().isatty() {
        args.skip(1)
            .flat_map(|n| n.split('\n'))
            .flat_map(|n| n.split(','))
            .filter_map(|n| n.parse::<f32>())
            .collect()
    } else {
        stdin_raw().read_to_string()
            .unwrap()
            .trim()
            .split(' ')
            .flat_map(|n| n.split('\n'))
            .flat_map(|n| n.split(','))
            .filter_map(|n| n.parse::<f32>())
            .collect()
    };

    println!("{}", spark::graph(values.as_slice()));
}
