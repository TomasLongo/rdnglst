use structopt::StructOpt;
use dialoguer::{Input, theme::ColorfulTheme};
use rusqlite::{Connection, Result};
use rusqlite::{NO_PARAMS, MappedRows, types::FromSql, types::FromSqlResult, types::ValueRef};

use std::io;
use std::io::Write;

use readinglist::{Backend, SqliteBackend, ReadingEntry, Format, formatEnumToString, stringToFormatEnum};

use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    Add
}

fn add() -> ReadingEntry {
    let title: String = create_promt_for(&String::from("Title"));
    let author: String = create_promt_for(&String::from("Author"));
    let status: String = create_promt_for(&String::from("Status"));
    let format: String = create_promt_for(&String::from("Format"));
    let genre: String = create_promt_for(&String::from("Genre"));
    let tags: String = create_promt_for(&String::from("Tags (space separated)"));

    let splits = tags.split(" ");

    // gibt es hierfuer nicht schon was in der std lib?
    let mut vec: Vec<String> = Vec::new();
    for s in splits {
        vec.push(String::from(s));
    }

    let re = ReadingEntry{
        id: -1000, // Not needed here
        author: author,
        title: title,
        format: stringToFormatEnum(&format),
        genre: genre,
        status: status,
        tags: vec
    };

    return re;
}

fn create_promt_for(item: &str) -> String {
    return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(item)
        .interact_text()
        .unwrap();
}

fn print_table(entries: &Vec<ReadingEntry>) -> Table {
    let mut table = Table::new();
    table
        .set_header(vec!["Title", "Author", "Genre", "Status", "Format"])
        .load_preset(UTF8_FULL);

    for e in entries {
        table.add_row(vec![
          &e.title,
          &e.author,
          &e.genre,
          &e.status,
          &formatEnumToString(&e.format)
        ]);
    }

    return table
}

fn main() -> Result<()>{
    let args = Cli::from_args();

    // let dbConn = init_db("./readinglist.db")?;
    let backend = SqliteBackend::new("./readinglist.db")?;
    match args.cmd {
        Some(Command::Add) => {
            let re = add();
            backend.addEntry(&re)
        },
        None => {
            let entries = backend.getAllEntries()?;

            println!("{}", print_table(&entries));
        }
    }

    Ok(())
}
