use clap::Parser;
use std::io::stdin;

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
    println!("Press Enter to open the browser (^C to quit)");
    stdin().read_line(&mut String::new())?;
    let hostname = hostname::get()?.to_str().ok_or("Invalid hostname")?;
    open::that("path")?;
    Ok(())
}