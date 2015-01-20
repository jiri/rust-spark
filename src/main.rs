extern crate spark;

fn main() {
    // println!("{}", spark::graph(&[1.0, 5.0, 22.0, 13.0, 53.0]));
    /* Should look like ▁▁▃▂█ */

    let values: Vec<f32> = std::os::args().iter()
        .skip(1)
        .map(|s| {
            match s.parse::<f32>() {
                Some(i) => i,
                None    => { println!("{}", s); 0.0 },
            }
        }).collect();

    // println!("{:?}", values);
    println!("{}", spark::graph(values.as_slice()));
}
