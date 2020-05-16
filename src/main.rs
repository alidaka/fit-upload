use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    arg1: String,
    arg2: String,
}

fn main() {
    let args = Cli::from_args();

    println!("{}", args.arg1);
    println!("{}", args.arg2);
}
