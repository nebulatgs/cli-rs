pub mod commands;
use clap::{AppSettings, ColorChoice, Parser};
use commands::*;

/// Interact with 🚅 Railway via CLI
/// Deploy infrastructure, instantly. Docs: https://docs.railway.app
#[derive(Parser)]
#[clap(
    color = ColorChoice::Never,
    name = "Railway CLI",
    version = "0.0.0",
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

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Add(a) => add::command(a),
        SubCommand::Connect(a) => connect::command(a),
        SubCommand::Delete(a) => delete::command(a),
        SubCommand::Docs(a) => docs::command(a),
        SubCommand::Environment(a) => environment::command(a),
        SubCommand::Init(a) => init::command(a),
        SubCommand::Link(a) => link::command(a),
        SubCommand::List(a) => list::command(a),
        SubCommand::Login(a) => login::command(a),
        SubCommand::Logout(a) => logout::command(a),
        SubCommand::Logs(a) => logs::command(a),
        SubCommand::Open(a) => open::command(a),
        SubCommand::Protect(a) => protect::command(a),
        SubCommand::Run(a) => run::command(a),
        SubCommand::Status(a) => status::command(a),
        SubCommand::Unlink(a) => unlink::command(a),
        SubCommand::Up(a) => up::command(a),
        SubCommand::Variables(a) => variables::command(a),
        SubCommand::Version(a) => version::command(a),
        SubCommand::Whoami(a) => whoami::command(a),
    }
}
