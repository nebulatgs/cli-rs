use clap::Parser;

#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	println!("🚝 Press Enter to open the browser (^C to quit)");
	let mut temp = String::new();
	std::io::stdin().read_line(&mut temp)?;
	match open::that("https://docs.railway.app/") {
		Ok(_) => Ok(()),
		Err(e) => Err(e.into()),
	}
}
