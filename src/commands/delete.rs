use clap::Parser;

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
	todo!("Delete command used!");
}
