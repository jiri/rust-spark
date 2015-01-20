#![allow(unstable)]

extern crate spark;
extern crate getopts;

use std::os;
use getopts::optflag;

/* TODO:
 *  - stdin support
 *  - show help
 *  - docs
 */

fn main() {
    let args = os::args();

    let opts = [
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
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.opt_present("h") {
        /* TODO: Print help here */
        println!("{}", getopts::usage("Echo the STRING(s) to standard output.", &opts).as_slice());
        return;
    }

    // println!("{:?}", args);

    let values: Vec<f32> = os::args().iter()
        .skip(1)
        .flat_map(|n| n.split(','))
        .filter_map(|n| n.parse::<f32>())
        .collect();

    // println!("{:?}", values);

    if !values.is_empty() {
        println!("{}", spark::graph(values.as_slice()));
    }
}
