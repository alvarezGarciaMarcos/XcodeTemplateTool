extern crate clap;
use clap::Parser;


#[derive(Parser)]
#[clap(version = "1.0", author = "alvarezGarciaMarcos")]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser)]
pub enum Commands {
    Setup(Setup),
    Template(Template),
}

// Setup

#[derive(Parser)]
/// Configure different aspects of the application
/// 
/// Configure both credentials and remotes using this function
pub struct Setup {
    #[clap(subcommand)]
    pub sub_command: SetupSubCommands,
}

#[derive(Parser)]
pub enum SetupSubCommands {
    Credential(Credential),
    Remote(Remote),
    /// Show the configured parameters
    Show
}

// Credential

#[derive(Parser)]
/// Actions related to Credentials
/// 
/// Allows to perform different actions related to Credentials, for instance: Add, Remove
pub struct Credential {
    #[clap(subcommand)]
    pub sub_command: CredsSubCommand
}

#[derive(Parser)]
pub enum CredsSubCommand {
    /// Adds a new credential
    Add(CredentialAddArgs),

    /// Removes a new credential
    Remove(CredentialRemoveArgs)
}

#[derive(Parser)]
pub struct CredentialAddArgs {
    pub username: String,
    pub password: String
}

#[derive(Parser)]
pub struct CredentialRemoveArgs {
    pub username: String
}

// Remote

#[derive(Parser)]
/// Allows to manage remotes
pub struct Remote {
    #[clap(subcommand)]
    pub sub_command: RemoteSubCommand
}

#[derive(Parser)]
pub enum RemoteSubCommand {
    /// Adds a new remote
    Add(RemoteAddArgs),

    /// Removes an existing remote
    Remove(RemoteRemoveArgs),
}

#[derive(Parser)]
pub struct RemoteAddArgs {
    #[clap()]
    pub remote: String,

    #[clap()]
    pub destination: Option<String>
}

#[derive(Parser)]
pub struct RemoteRemoveArgs {
    #[clap()]
    pub remote: String,
}

// Template
#[derive(Parser)]
/// Manage your templates
/// 
/// Allows to fetch, install and list all available templates
pub struct Template {
    #[clap(subcommand)]
    pub sub_command: TemplateSubCommand
}

#[derive(Parser)]
pub enum TemplateSubCommand {
    Install(InstallArgs),
    Uninstall(UninstallArgs),
    /// Fetches changes from the remote
    Fetch,
    /// List all the templates in the remote
    List,
}

// Install

#[derive(Parser)]
/// Install new templates
/// 
/// Install new templates fetched from the remote
pub struct InstallArgs {
    #[clap()]
    pub template: Option<String>,

    #[clap(short, long, default_value="true")]
    /// Install as a file template (true by default)
    pub file: bool,
}

#[derive(Parser)]
/// Uninstall a template
/// 
/// Uninstall already installed templates
pub struct UninstallArgs {
    #[clap()]
    pub template: Option<String>,

    #[clap(short, long, default_value="false")]
    pub all: bool,
}
