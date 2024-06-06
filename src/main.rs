mod colour;
mod grid;
mod pixels;
mod softbuffer;

use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    frontend: Frontend,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct Frontend {
    /// use `pixels` frontend
    #[arg(long, default_value_t = true)]
    pixels: bool,

    /// use `shortbuffer` frontend
    #[arg(long)]
    shortbuffer: bool,
}

fn main() {
    let cli = Cli::parse();
    let frontend = &cli.frontend;
    let main = if frontend.pixels {
        pixels::main
    } else if frontend.shortbuffer {
        softbuffer::main
    } else {
        panic!()
    };
    let mut s = grid::Grid::new(100, 100, 0);
    main(&mut s);
}
