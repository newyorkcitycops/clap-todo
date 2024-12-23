#![allow(dead_code)]
use clap::Parser;
use rusqlite::{Connection, Result};
use todo::{TodoCli, TodoCliSubCommands};

struct Todo {
    id: i32,
    title: String,
    done: bool,
  }
  
impl Todo {
    fn add(id: i32, title: &str, done: bool) -> Self {
        Self {
            id,
            title: title.to_string(),
            done,
        }
    }
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
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }

    Ok(todos)
}

fn main() -> Result<()> {
    let args = TodoCli::parse();
    let connection = Connection::open("todo.sqlite")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
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
        None => {
            let todos = get_todos(&connection)?;
            for todo in todos {
                println!("{}", todo.title)
            }
        }
    }

    Ok(())
}