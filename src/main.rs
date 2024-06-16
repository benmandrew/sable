mod colour;
mod grid;
mod one_shot;
mod pixels;
mod softbuffer;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value_t = 100)]
    width: usize,

    #[arg(long, default_value_t = 100)]
    height: usize,

    #[arg(short = 't', long = "threads", default_value_t = 4)]
    n_threads: usize,

    #[arg(long, group = "colour")]
    rgb_continuous: bool,

    #[arg(long, group = "colour", default_value_t = true)]
    rgb_discrete: bool,
}

#[derive(Subcommand)]
enum Commands {
    Realtime(RealtimeArgs),
    Bmp(BmpArgs),
    Terminal(TerminalArgs),
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct RealtimeArgs {
    /// Use `pixels` frontend
    #[arg(long)]
    #[arg(default_value_t = true)]
    pixels: bool,

    /// Use `softbuffer` frontend
    #[arg(long)]
    softbuffer: bool,
}

#[derive(Args)]
struct BmpArgs {
    #[arg(short, long, default_value_t = String::from("out.bmp"))]
    output: String,

    #[arg(short = 'i', long = "iterations")]
    n_iterations: usize,
}

#[derive(Args)]
struct TerminalArgs {
    #[arg(short = 'i', long = "iterations")]
    n_iterations: usize,
}

fn get_convert_colour(cli: &Cli) -> fn(f64) -> u32 {
    if cli.rgb_continuous {
        colour::hsv_to_rgb
    } else if cli.rgb_discrete {
        colour::discrete_rgb
    } else {
        panic!("Colour conversion function is not set")
    }
}

fn main() {
    let cli = Cli::parse();
    let convert_colour = get_convert_colour(&cli);
    let mut g = grid::Grid::new(cli.width, cli.height, cli.n_threads, 0, convert_colour);
    match &cli.command {
        Commands::Realtime(cmd) => {
            if cmd.pixels {
                println!("Using 'pixels' frontend");
                pixels::main(&mut g)
            } else if cmd.softbuffer {
                println!("Using 'softbuffer' frontend");
                softbuffer::main(&mut g)
            } else {
                panic!("Frontend is not set")
            }
        }
        Commands::Bmp(cmd) => {
            one_shot::main_bmp(&mut g, cmd.n_iterations, cmd.output.as_str())
        }
        Commands::Terminal(cmd) => {
            one_shot::main_terminal(&mut g, cmd.n_iterations)
        }
    };
}
