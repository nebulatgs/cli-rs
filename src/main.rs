pub mod commands;
pub mod gql;
pub mod util;

use clap::{AppSettings, ColorChoice, Parser};
use commands::*;

use crate::util::{consts, errors::RailwayError};
/// Interact with 🚅 Railway via CLI
/// Deploy infrastructure, instantly. Docs: https://docs.railway.app
#[derive(Parser)]
#[clap(
    color = ColorChoice::Never,
    name = "Railway CLI",
    version = consts::VERSION,
    verbatim_doc_comment,
    setting = AppSettings::SubcommandRequiredElseHelp,
    after_help = "Use \"railway [command] --help\" for more information about a command."
)]
struct Opts {
	#[clap(subcommand, name = "command")]
	subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
	#[clap(about = "Add a new plugin to your project")]
	Add(add::Args),
	//   #[clap(about = "Generate completion script")]
	//   Completion(completion::Args),
	#[clap(about = "Open an interactive shell to a database")]
	Connect(connect::Args),
	#[clap(about = "Delete Project, may specify projectId as an argument")]
	Delete(delete::Args),
	#[clap(about = "Open Railway Documentation in default browser")]
	Docs(docs::Args),
	#[clap(about = "Change the active environment")]
	Environment(environment::Args),
	#[clap(about = "Create a new Railway project")]
	Init(init::Args),
	#[clap(
		about = "Associate existing project with current directory, may specify projectId as an argument"
	)]
	Link(link::Args),
	#[clap(about = "List all projects in your Railway account")]
	List(list::Args),
	#[clap(about = "Login to your Railway account")]
	Login(login::Args),
	#[clap(about = "Logout of your Railway account")]
	Logout(logout::Args),
	#[clap(about = "View the most-recent deploy's logs")]
	Logs(logs::Args),
	#[clap(about = "Open your project dashboard")]
	Open(open::Args),
	#[clap(about = "[EXPERIMENTAL!] Protect current branch (Actions will require confirmation)")]
	Protect(protect::Args),
	#[clap(about = "Run a local command using variables from the active environment")]
	Run(run::Args),
	#[clap(about = "Show information about the current project")]
	Status(status::Args),
	#[clap(about = "Disassociate project from current directory")]
	Unlink(unlink::Args),
	#[clap(about = "Upload and deploy project from the current directory")]
	Up(up::Args),
	#[clap(about = "Show variables for active environment")]
	Variables(variables::Args),
	#[clap(about = "Get the version of the Railway CLI")]
	Version(version::Args),
	#[clap(about = "Get the current logged in user")]
	Whoami(whoami::Args),
}

#[tokio::main]
async fn main() {
	let opts: Opts = Opts::parse();
	let res = match opts.subcmd {
		SubCommand::Add(a) => add::command(a).await,
		SubCommand::Connect(a) => connect::command(a).await,
		SubCommand::Delete(a) => delete::command(a).await,
		SubCommand::Docs(a) => docs::command(a).await,
		SubCommand::Environment(a) => environment::command(a).await,
		SubCommand::Init(a) => init::command(a).await,
		SubCommand::Link(a) => link::command(a).await,
		SubCommand::List(a) => list::command(a).await,
		SubCommand::Login(a) => login::command(a).await,
		SubCommand::Logout(a) => logout::command(a).await,
		SubCommand::Logs(a) => logs::command(a).await,
		SubCommand::Open(a) => open::command(a).await,
		SubCommand::Protect(a) => protect::command(a).await,
		SubCommand::Run(a) => run::command(a).await,
		SubCommand::Status(a) => status::command(a).await,
		SubCommand::Unlink(a) => unlink::command(a).await,
		SubCommand::Up(a) => up::command(a).await,
		SubCommand::Variables(a) => variables::command(a).await,
		SubCommand::Version(a) => version::command(a).await,
		SubCommand::Whoami(a) => whoami::command(a).await,
	};
	let cmd_name = std::env::args().nth(1);
	if let Some(name) = cmd_name {
		if let Err(s) = res {
			match s {
				RailwayError::ProjectNotFound => {
					eprintln!("{}", s)
				}
				RailwayError::Unauthorized => {
					eprintln!("{}", s)
				}
				RailwayError::NotLinked => {
					eprintln!("{}", s)
				}
				RailwayError::EnvironmentNotFound => {
					eprintln!("{}", s)
				}
				RailwayError::Panic(err, _backtrace) => {
					eprintln!("Unexpected Error! {}", err);
					todo!("Send error back to HQ")
				}
				_ => {
					eprintln!("Error in command \'{}\': {}", name, s)
				}
			}
		}
	}
}
