#[derive(clap::Parser, Debug)]
pub struct NewCommand {
    input: String,
}

impl NewCommand {
    pub fn run(&self) {
        println!("Running new command with input: {}", self.input);
    }
}
