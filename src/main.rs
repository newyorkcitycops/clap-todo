use clap::Parser;
use rusqlite::{Connection, Result, ToSql};
use todo::{TodoColumns, TodoCli, TodoCliSubCommands};
use tabled::{Tabled, Table};

#[derive(Eq, Ord, PartialEq, PartialOrd, Tabled)]
struct Todo {
    id: i64,
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

fn search_todos(connection: &Connection, query: String) -> Result<Vec<Todo>> {
    let mut statement = match query.parse::<i64>() {
        Ok(_) => connection.prepare(
            "SELECT id, title, done FROM todo WHERE id = ?1"
        )?,
        Err(_) => connection.prepare(  
            "SELECT id, title, done FROM todo WHERE title LIKE ?1"
        )?,
    };   

    let query_params: [&dyn ToSql; 1] = match query.parse::<i64>() {
        Ok(id) => [&id.to_string()],
        Err(_) => [&format!("%{}%", query) as &dyn ToSql],
    };

    let todo_mapper =
        |row: &rusqlite::Row | -> Result<Todo> {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                done: row.get(2)?,
            })
        };

    let todo_iter =
        statement.query_map(&query_params, todo_mapper)?;

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

    if let Some(todo) = args.todo {
        println!("{}", Table::new(search_todos(&connection, todo)?).to_string());
    } else if let Some(sort) = args.sort {
        let mut todos = get_todos(&connection)?;

        match sort {
            TodoColumns::Id => todos.sort_by(
                |a, b| b.id.cmp(&a.id)
            ),
            TodoColumns::Title => todos.sort_by(
                |a, b|
                a.title.to_lowercase().cmp(&b.title.to_lowercase())
            ),
            TodoColumns::Done => todos.sort_by_key(|todo| todo.done),
        }

        println!("{}", Table::new(todos).to_string());
    } else if args.subcommand.is_none() {
        println!("{}", Table::new(get_todos(&connection)?).to_string());
    } else if let Some (subcommand) = args.subcommand {
        match subcommand {
            TodoCliSubCommands::Add(add_todo_arg) => {
                for todo in add_todo_arg.todos {
                    connection.execute(
                        "INSERT INTO todo (title) VALUES (?1)",
                        &[&todo],
                    )?;
                }
            }
            TodoCliSubCommands::Remove(remove_todo_arg) => {
                for todo in remove_todo_arg.todos {
                    match todo.parse::<i64>() {
                        Ok(_) => connection.execute(
                            "DELETE FROM todo WHERE id = ?1",
                            &[&todo],
                        )?,
                        Err(_) => connection.execute(
                            "DELETE FROM todo WHERE title = ?1",
                            &[&todo],
                        )?,
                    };
                }
            }
            TodoCliSubCommands::Done(done_todo_arg) => {
                for todo in done_todo_arg.todos {
                    match todo.parse::<i64>() {
                        Ok(_) => connection.execute(
                            "UPDATE todo SET done = NOT done WHERE id = ?1",
                            &[&todo],
                        )?,
                        Err(_) => connection.execute(
                            "UPDATE todo SET done = NOT done WHERE title = ?1",
                            &[&todo],
                        )?,
                    };
                }
            }
        }
    }
    
    Ok(())
}