#![allow(unstable)]

extern crate spark;
extern crate getopts;

use std::os;
use getopts::optflag;

/* TODO:
 *  - stdin support
 *  - docs
 */

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
        println!("spark v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    /* XXX: This could be generated better */
    if matches.opt_present("h") {
        println!("{}", getopts::usage("Usage:\n\tspark [-hv] VALUE,...", &opts).as_slice());
        return;
    }

    let values: Vec<f32> = os::args().iter()
        .skip(1)
        .flat_map(|n| n.split(','))
        .filter_map(|n| n.parse::<f32>())
        .collect();

    if !values.is_empty() {
        println!("{}", spark::graph(values.as_slice()));
    }
}
