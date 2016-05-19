extern crate getopts;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut opts = getopts::Options::new();
    opts.optflag("n", "", "do not output the trailing newline");
    let matches = opts.parse(&args);

    match matches {
        Ok(m) => {
            show(&m.free);

            if !m.opt_present("n") {
                print!("\n");
            }
        }
        Err(_) => {
            show(args);
            print!("\n");
        }
    };
}

fn show<C: IntoIterator>(args: C)
    where C::Item: std::fmt::Display
{
    let mut i = 0;
    for arg in args {
        if i > 0 {
            print!(" ");
        }
        i += 1;
        print!("{}", arg);
    }
}