use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct AddTodoArgument {
  pub todos: Vec<String>,
}

#[derive(Parser)]
pub struct RemoveTodoArgument {
  pub todos: Vec<String>,
}

#[derive(Parser)]
pub struct DoneTodoArgument {
  pub todos: Vec<String>,
}

#[derive(Parser)]
pub struct SortTodoArgument {
  pub sort: TodoColumns,
}

#[derive(Parser)]
pub struct ViewTodoArgument {
  pub todo: String,
}

pub enum TodoCliCommands {
  Sort(SortTodoArgument),
  View(ViewTodoArgument),
}

pub enum TodoCliSubCommands {
  Add(AddTodoArgument),
  Remove(RemoveTodoArgument),
  Done(DoneTodoArgument),
}

#[derive(ValueEnum, Clone)]
pub enum TodoColumns {
  Id,
  Title,
  Done,
}

#[derive(Parser)]
#[command(subcommand_negates_reqs = true, subcommand_required = false)]
pub struct TodoCli {
  #[command(subcommand)]
  pub subcommand: Option<TodoCliSubCommands>,
  #[arg(short, long)]
  pub sort: Option<TodoColumns>,
  #[arg(short, long)]
  pub search: Option<String>,
}

impl FromArgMatches for TodoCliSubCommands {
  fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
    match matches.subcommand() {
      Some(("add", args)) => Ok(Self::Add(
        AddTodoArgument::from_arg_matches(args)?
      )),
      Some(("remove", args)) => Ok(Self::Remove(
        RemoveTodoArgument::from_arg_matches(args)?
      )),
      Some(("done", args)) => Ok(Self::Done(
        DoneTodoArgument::from_arg_matches(args)?
      )),
      Some((_, _)) => Err(Error::raw(
        ErrorKind::InvalidSubcommand,
        "Valid subcommands are `add`, `remove` and `done`",
      )),
      None => Err(Error::raw(
        ErrorKind::MissingSubcommand,
        "Valid subcommands are `add`, `remove` and `done`", 
      )),
    }
  }

  fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
      match matches.subcommand() {
        Some(("add", args)) => *self = Self::Add(
          AddTodoArgument::from_arg_matches(args)?
        ),
        Some(("remove", args)) => *self = Self::Remove(
          RemoveTodoArgument::from_arg_matches(args)?
        ),
        Some(("done", args)) => *self = Self::Done(
          DoneTodoArgument::from_arg_matches(args)?
        ),
        Some((_, _)) => {
          return Err(Error::raw(
            ErrorKind::InvalidSubcommand,
            "Valid subcommands are `add`, `remove` and `done`"
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
        AddTodoArgument::augment_args(Command::new("add"))
      )
      .subcommand(
        RemoveTodoArgument::augment_args(Command::new("remove"))
      )
      .subcommand(
        DoneTodoArgument::augment_args(Command::new("done"))
      )
      .subcommand_required(true)
  }

  fn augment_subcommands_for_update(cmd: Command) -> Command {
      cmd.subcommand(
        AddTodoArgument::augment_args(Command::new("add"))
      )
      .subcommand(
        RemoveTodoArgument::augment_args(Command::new("remove"))
      )
      .subcommand(
        DoneTodoArgument::augment_args(Command::new("done"))
      )
      .subcommand_required(true)
  }

  fn has_subcommand(name: &str) -> bool {
      matches!(name, "add" | "remove" | "done")
  }
}
