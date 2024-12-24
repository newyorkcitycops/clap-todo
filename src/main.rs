#![allow(dead_code)]
use clap::Parser;
use rusqlite::{Connection, Result};
use todo::{TodoCli, TodoCliSubCommands};
use tabled::{Tabled, Table};

#[derive(Tabled)]
struct Todo {
    id: i32,
    title: String,
    #[tabled(display_with = "format_done")]
    done: bool,
}

fn get_todos(connection: &Connection) -> Result<Vec<Todo>> {
    let mut statement = connection.prepare(
        "SELECT id, title, done FROM todo"
    )?;
    let todo_iter = statement.query_map(
        [], 
        |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                done: row.get(2)?,
            })
        }
    )?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }

    Ok(todos)
}

fn format_done(done: &bool) -> String {
    match done {
        false => "no".to_string(),
        true => "yes".to_string(),
    }
}

fn main() -> Result<()> {
    let args = TodoCli::parse();
    let connection: Connection = Connection::open("todo.sqlite")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT UNIQUE NOT NULL,
            done INTEGER DEFAULT 0
        )",
        (),
    )?;

    match args.subcommand {
        Some(TodoCliSubCommands::Add(add_todo_args)) => {
            for todo in add_todo_args.todos {
                connection.execute(
                    "INSERT INTO todo (title) VALUES (?1)",
                    &[&todo],
                )?;
            }
        }
        Some(TodoCliSubCommands::Remove(remove_todo_args)) => {
            for todo in remove_todo_args.todos {
                connection.execute(
                    "DELETE FROM todo WHERE title = (?1)",
                    &[&todo],
                )?;
            }
        }
        Some(TodoCliSubCommands::Done(done_todo_args)) => {
            for todo in done_todo_args.todos {
                connection.execute(
                    "UPDATE todo SET done = NOT done WHERE title = (?1)",
                    &[&todo],
                )?;
            }
        }
        None => println!("{}", Table::new(get_todos(&connection)?).to_string())
    }

    Ok(())
}