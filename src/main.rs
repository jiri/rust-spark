#![allow(unstable)]

extern crate spark;

/* TODO:
 *  - stdin support
 *  - show help
 *  - docs
 */
fn main() {
    let values: Vec<f32> = std::os::args().iter()
        .skip(1)
        .flat_map(|x| x.split(','))
        .flat_map(|x| x.split(';'))
        .filter(|x| x.len() != 0)
        .map(|s| {
            match s.parse::<f32>() {
                Some(i) => i,
                None    => { println!("{}", s); 0.0 },
            }
        })
        .collect();

    println!("{}", spark::graph(values.as_slice()));
}
