#![allow(dead_code)]
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct AddTodoArgument {
  pub todos: Vec<String>,
}

#[derive(Debug)]
pub enum TodoCliSubCommands {
  Add(AddTodoArgument),
}

#[derive(Debug, Parser)]
#[command(subcommand_negates_reqs = true, subcommand_required = false)]
pub struct TodoCli {
  #[command(subcommand)]
  pub subcommand: Option<TodoCliSubCommands>,
}

impl FromArgMatches for TodoCliSubCommands {
  fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
    match matches.subcommand() {
      Some(("add", args)) => Ok(Self::Add(
        AddTodoArgument::from_arg_matches(args)?
      )),
      Some((_, _)) => Err(Error::raw(
        ErrorKind::InvalidSubcommand,
        "Valid subcommands are `add`",
      )),
      None => Err(Error::raw(
        ErrorKind::MissingSubcommand,
        "Valid subcommands are `add`", 
      )),
    }
  }

  fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
      match matches.subcommand() {
        Some(("add", args)) => *self = Self::Add(
          AddTodoArgument::from_arg_matches(args)?
        ),
        Some((_, _)) => {
          return Err(Error::raw(
            ErrorKind::InvalidSubcommand,
            "Valid subcommands are `add`"
          ))
        },
        None => (),
      };

      Ok(())
  }
}

impl Subcommand for TodoCliSubCommands {
  fn augment_subcommands(cmd: Command) -> Command {
      cmd.subcommand(
        AddTodoArgument::augment_args(Command::new("add")
      ))
      .subcommand_required(true)
  }

  fn augment_subcommands_for_update(cmd: Command) -> Command {
      cmd.subcommand(
        AddTodoArgument::augment_args(Command::new("add")
      ))
      .subcommand_required(true)
  }

  fn has_subcommand(name: &str) -> bool {
      matches!(name, "add")
  }
}
