use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "nidrs-cli",
    version = "0.0.1",
    author = "Wuma",
    about = "nidrs cli helper."
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new project
    New {
        /// Input file
        #[clap()]
        input: String,
    },

    /// run a project.
    Start {
        #[clap(help = "Output file")]
        output: String,
    },

    /// build a project.
    Build {
        #[clap(help = "Output file")]
        output: String,
    },

    /// generate module\controller\service\interceptor\dto code.
    Gen {
        #[clap(help = "Output file")]
        output: String,
    },

    /// gpt generate code.
    Chat {
        #[clap(help = "Output file")]
        output: String,
    },

    /// publish a nidrs module.
    Publish {
        #[clap(help = "Output file")]
        output: String,
    },

    /// install a nidrs module.
    Install {
        #[clap(help = "Output file")]
        output: String,
    },

    /// print nidrs info.
    Print {
        #[clap(help = "Output file")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // match &cli.command {
    //     Commands::Sub1 { input } => {
    //         println!("Running subcommand 1 with input: {}", input);
    //         // 在这里执行子命令 1 的逻辑
    //     }
    //     Commands::Sub2 { output } => {
    //         println!("Running subcommand 2 with output: {}", output);
    //         // 在这里执行子命令 2 的逻辑
    //     }
    // }
}
