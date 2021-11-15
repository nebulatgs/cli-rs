use clap::Parser;

#[derive(Parser)]
pub struct Args;

pub fn command(args: Args) -> super::CommandResult {
    todo!("Whoami command used!");
}