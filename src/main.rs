extern crate clap;
mod interface;
mod core;
mod config;
mod security;
mod error;
mod constants;
mod repo;
mod storage;

use clap::Parser;
use interface::{CLI, Commands::*, TemplateSubCommand, SetupSubCommands, CredsSubCommand, RemoteSubCommand};
use security::{setup_credentials, remove_credentials};
use core::{fetch, add_remote, remove_remote, list, install_all, install, show_configuration, uninstall, uninstall_all};

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Setup(setup)) => match &setup.sub_command {
            SetupSubCommands::Credential(credential) => match &credential.sub_command {
                CredsSubCommand::Add(args) => setup_credentials(&args.username, &args.password),
                CredsSubCommand::Remove(args) => remove_credentials(&args.username)
            }
            // add_remote(&args.remote),
            SetupSubCommands::Remote(remote) => match &remote.sub_command {
                RemoteSubCommand::Add(args) => match add_remote(&args.remote, &args.destination) {
                    Ok((remote, local)) => println!("Repo '{}' cloned in '{}'", remote, local),
                    Err(error) => println!("Error: {}", error)
                }
                RemoteSubCommand::Remove(args) => remove_remote(&args.remote)
            }
            SetupSubCommands::Show => match show_configuration() {
                Ok(configuration_string) => println!("{}", configuration_string),
                Err(error) => println!("Error: {}", error)
            }
            // remove_remote(&args.remote),
        }

        Some(Template(template)) => match &template.sub_command {
            TemplateSubCommand::Fetch => fetch(),

            TemplateSubCommand::Install(args) =>   match &args.template {
                Some(template) => match install(template, args.file) {
                    Ok(()) => println!("Template '{}' installed", template),
                    Err(error) => println!("Error installing template: {}", error)
                }
                None => install_all()
            }

            TemplateSubCommand::Uninstall(args) => match &args.template {
                Some(template) => match uninstall(template) {
                    Ok(_) => println!("Template '{}' successfully uninstalled", template),
                    Err(error) => println!("{}", error)
                }

                None => match uninstall_all() {
                    Ok(_) => println!("All templates have been correctly uninstalled"),
                    Err(error) => println!("{}", error)
                }
            }

            TemplateSubCommand::List => list(),
        }

        None => println!("No command provided"),
    }
}
