use clap::Parser;

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
	todo!("Run command used!");
}
