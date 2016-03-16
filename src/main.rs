extern crate spark;

use std::io::{stdin, Read};

fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if args.iter().any(|s| s.contains("-v")) {
        println!("spark-rs v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.iter().any(|s| s.contains("-h")) {
        println!("Usage:\n\tspark [-hv] VALUE,...");
        return;
    }

    let values: Vec<f32> = if args.len() == 0 {
        let mut buffer = String::new();
        match stdin().read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't read from stdin!"),
        };

        buffer.split_whitespace()
            .flat_map(|n| n.split(','))
            .filter_map(|n| n.parse::<f32>().ok())
            .collect()
    } else {
        args.iter()
            .flat_map(|n| n.split(','))
            .filter_map(|n| n.parse::<f32>().ok())
            .collect()
    };

    println!("{}", spark::graph(values.as_slice()));
}
