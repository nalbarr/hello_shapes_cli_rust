use clap::Parser;
use shapes_cli::create_shapes;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let num_shapes: usize = opts.number;
    create_shapes(num_shapes);

    println!(
        "Created shapes with {} shapes: {:?}",
        num_shapes,
        create_shapes(num_shapes)
    );
}
