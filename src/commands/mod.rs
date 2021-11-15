pub mod add;
pub mod docs;
pub mod version;
pub mod connect;
pub mod delete;
pub mod environment;
pub mod init;
pub mod link;
pub mod list;
pub mod login;
pub mod logout;
pub mod logs;
pub mod open;
pub mod protect;
pub mod run;
pub mod status;
pub mod unlink;
pub mod up;
pub mod variables;
pub mod whoami;

type CommandResult = Result<(), Box<dyn std::error::Error>>;