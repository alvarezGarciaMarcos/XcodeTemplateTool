use clap::{Parser, arg, Command };

// pub trait ExtendClapCommand {
//     fn create_all_subcommands() -> Command;
// }

// impl ExtendClapCommand for clap::Command {
//     // fn create_all_subcommands() -> Command {
//         // self.subcommand(
//         //     clap::Command::new("add")
//         //     .about("Allows to add files")
//         //     .arg(arg!([FILE]))
//         // )
//         // .subcommand(
//         //     clap::Command::new("remove")
//         //     .about("Allows to remove files")
//         //     .arg(arg!([FILE]))
//         // )
//     }
// }


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_enum)]
    action: Action,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


#[derive(Debug, PartialEq, Clone,clap::ValueEnum)]
pub enum Action {
    SETUP,
    UPDATE
}