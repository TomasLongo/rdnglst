use structopt::StructOpt;
use dialoguer::{Input, theme::ColorfulTheme};
use rusqlite::{Connection, Result};
use rusqlite::{NO_PARAMS, MappedRows, types::FromSql, types::FromSqlResult, types::ValueRef};

use std::io;
use std::io::Write;

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

#[derive(Debug)]
enum Format {
    Book,
    Kindle
}

impl FromSql for Format {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(s) => {
                return Ok(Format::Book);
            },
            _ => return Ok(Format::Kindle),
        }
    }
}

#[derive(Debug)]
struct ReadingEntry {
    id: i64,
    author: String,
    title: String,
    genre: String,
    format: Format,
    status: String,
    tags: Vec<String>
}

fn stringToFormatEnum(toConvert: &str) -> Format {
    let lc = toConvert.to_lowercase();
    if lc == "book" {
        return Format::Book;
    } else if lc == "kindle" {
        return Format::Kindle
    } else {
        return Format::Book
    }
}

fn formatEnumToString(f: &Format) -> String {
    match f {
        Format::Book => return String::from("book"),
        Format::Kindle => return String::from("kindle")
    };
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

fn query_reading_list(conn: &Connection) -> Result<Vec<ReadingEntry>> {
    let mut stmt = conn.prepare("SELECT id, title, author, genre, format, tags, status FROM reading_entries")?;

    let entries = stmt.query_map(NO_PARAMS, |row| {

        let tags_from_db: String = row.get(5)?;
        let splits = tags_from_db.split(" ");

        // gibt es hierfuer nicht schon was in der std lib?
        let mut vec: Vec<String> = Vec::new();
        for s in splits {
            vec.push(String::from(s));
        }

        return Ok(ReadingEntry {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            genre: row.get(3)?,
            format: row.get(4)?,
            status: row.get(6)?,
            tags: vec
        })
    })?;

    let mut entryList = Vec::new();
    for e in entries {
        entryList.push(e?);
    }

    return Ok(entryList);
}

fn create_promt_for(item: &str) -> String {
    return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(item)
        .interact_text()
        .unwrap();
}

fn init_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        " create table if not exists reading_entries (
            id integer primary key autoincrement,
            title text not null default '',
            author text not null default '',
            genre text  not null default '',
            format text not null default '',
            tags text not null default '',
            status text not null default '',
            created_at timestamp default current_timestamp,
            updated_at timestamp default current_timestamp
            );
        ", NO_PARAMS,)?;

    return Ok(conn);
}

fn insertReadingEntry(re: &ReadingEntry, conn: &Connection) {
    let insertString = "insert into reading_entries (title, author, genre, format, tags, status) values (?1, ?2, ?3, ?4, ?5, ?6);";
    conn.execute(insertString, &[&re.title, &re.author, &re.genre, &formatEnumToString(&re.format), &re.tags.join(" "), &re.status]);
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

    let dbConn = init_db("./readinglist.db")?;
    match args.cmd {
        Some(Command::Add) => {
            let re = add();
            insertReadingEntry(&re, &dbConn)
        },
        None => {
            let entries = query_reading_list(&dbConn)?;

            println!("{}", print_table(&entries));
        }
    }

    Ok(())
}
