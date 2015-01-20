#![allow(unstable)]

extern crate spark;
extern crate getopts;

/* TODO: Sort these out better */
use std::os;
use getopts::optflag;
use std::io::stdio::stdin_raw;

/* TODO: docs */
fn main() {
    let args = os::args();

    let opts = [
        /* TODO: More possible flags */
        /* - Wait for EOF to draw */
        /* - Set min / max values */
        optflag("h", "help", "display this help text"),
        optflag("v", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            os::set_exit_status(1);
            return;
        }
    };

    if matches.opt_present("v") {
        println!("spark-rs v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    /* FIXME: This should be generated better */
    if matches.opt_present("h") {
        println!("{}", getopts::usage("Usage:\n\tspark [-hv] VALUE,...", &opts).as_slice());
        return;
    }

    /* XXX: This is becoming a bit too much */
    let values: Vec<f32> = if stdin_raw().isatty() {
        os::args().iter().skip(1)
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

    if !values.is_empty() {
        println!("{}", spark::graph(values.as_slice()));
    } else {
        println!("{}", getopts::usage("Usage:\n\tspark [-hv] VALUE,...", &opts).as_slice());
    }
}
