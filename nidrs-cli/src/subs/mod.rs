mod new;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// create a new project
    New(new::NewCommand),

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

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::New(new) => new.run(),
            _ => println!("Running command"),
        }
    }
}
