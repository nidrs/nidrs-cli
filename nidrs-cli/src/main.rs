use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::{output::OutputOkExt, Command};

    #[test]
    fn test_hello() {
        let mut cmd = Command::cargo_bin("nidrs-cli").unwrap();
        cmd.arg("--name").arg("Alice").arg("--count").arg("3");
        cmd.assert().success();
        // insta::assert_debug_snapshot!(cmd.output());
    }
}
