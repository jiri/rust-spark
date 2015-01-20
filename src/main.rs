extern crate spark;

fn main() {
    println!("{}", spark::graph(&[1.0, 5.0, 22.0, 13.0, 53.0]));
    /* Should look like ▁▁▃▂█ */
}
