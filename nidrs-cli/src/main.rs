mod flow;
mod shared;
mod subs;

use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "nidrs-cli",
    version = "0.0.1",
    author = "Wuma",
    about = "nidrs cli helper."
)]
struct Cli {
    #[clap(subcommand)]
    command: subs::Commands,
}

fn main() {
    let cli = Cli::parse();

    cli.command.run();
}
