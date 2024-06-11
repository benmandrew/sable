mod colour;
mod grid;
mod one_shot;
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
#[group(required = true, multiple = false)]
struct Frontend {
    /// use `pixels` frontend
    #[arg(long)]
    pixels: bool,

    /// use `softbuffer` frontend
    #[arg(long)]
    softbuffer: bool,

    /// use `one-shot` frontend
    #[arg(long)]
    one_shot: bool,
}

fn main() {
    let cli = Cli::parse();
    let frontend = &cli.frontend;
    let main = if frontend.pixels {
        println!("Using 'pixels' frontend");
        pixels::main
    } else if frontend.softbuffer {
        println!("Using 'softbuffer' frontend");
        softbuffer::main
    } else if frontend.one_shot {
        println!("Using 'one-shot' frontend");
        one_shot::main
    } else {
        panic!()
    };
    let mut s = grid::Grid::new(400, 400, 0);
    main(&mut s);
}
