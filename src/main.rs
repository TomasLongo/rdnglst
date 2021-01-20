// insight: mod keyword wird auch als import direktive verwendet
mod querylanguage;
mod config;
mod log;

use structopt::StructOpt;
use dialoguer::{Input, theme::ColorfulTheme};
use rusqlite::{Connection, Result};

// Insight: Damit der Compiler Trait-Methoden an einer Struct aufrufen kann, muss das Trait
// mit importiert werden.
use readinglist::{Backend, SqliteBackend, ReadingEntry, formatEnumToString, stringToFormatEnum};

use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

use crate::querylanguage::{eval, parse_query, Modifier, TableRow};
use crate::config::Config;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = {
        return initConfig();
    };
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    #[structopt(short="i", long = "id")]
    withId: bool,

    #[structopt(short="x", long = "debug")]
    debug: bool,

    #[structopt(short="q", long = "query", default_value="")]
    q: String,

    #[structopt(long = "db-file", default_value="~/.config/readinglist/readinglist.db")]
    db_file_location: String
}

#[derive(StructOpt)]
enum Command {
    Add,

    Update {
        #[structopt(long = "id")]
        id: i64
    },

    Rm {
        #[structopt(long = "id")]
        id: i64
    }
}

fn promptForUpdate(toUpdate: &ReadingEntry) -> ReadingEntry {
    let title: String = create_promt_for(&String::from("Title"), Some(&toUpdate.title));
    let author: String = create_promt_for(&String::from("Author"), Some(&toUpdate.author));
    let status: String = create_promt_for(&String::from("Status"), Some(&toUpdate.status));
    let format: String = create_promt_for(&String::from("Format"), Some(&formatEnumToString(&toUpdate.format)));
    let genre: String = create_promt_for(&String::from("Genre"), Some(&toUpdate.genre));
    let tags: String = create_promt_for(&String::from("Tags (space separated)"), Some(&toUpdate.tags.join(" ")));

    let splittedTags = tags.split(" ").map(|x| String::from(x)).collect();

    let re = ReadingEntry{
        id: toUpdate.id, // Not needed here
        author: author,
        title: title,
        format: stringToFormatEnum(&format),
        genre: genre,
        status: status,
        tags: splittedTags
    };

    return re;
}

fn add() -> ReadingEntry {
    let title: String = create_promt_for(&String::from("Title"), None);
    let author: String = create_promt_for(&String::from("Author"), None);
    let status: String = create_promt_for(&String::from("Status"), None);
    let format: String = create_promt_for(&String::from("Format"), None);
    let genre: String = create_promt_for(&String::from("Genre"), None);
    let tags: String = create_promt_for(&String::from("Tags (space separated)"), None);

    let splits = tags.split(" ");

    let splittedTags = tags.split(" ").map(|x| String::from(x)).collect();

    let re = ReadingEntry{
        id: -1000, // Not needed here
        author: author,
        title: title,
        format: stringToFormatEnum(&format),
        genre: genre,
        status: status,
        tags: splittedTags
    };

    return re;
}

fn create_promt_for(item: &str, initial_text: Option<&String>) -> String {
    return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(item)
        .with_initial_text(initial_text.unwrap_or(&"".to_string()))
        .interact_text()
        .unwrap();
}

fn print_table(entries: &Vec<ReadingEntry>, withId: bool) -> Table {
    let mut headers = vec!["Title", "Author", "Genre", "Status", "Format", "Tags"];
    if withId {
        headers.push("ID")
    }

    let mut table = Table::new();
    table
        .set_header(headers)
        .load_preset(UTF8_FULL);

    for e in entries {
        let tags = e.tags.join(" ");
        let formatString = formatEnumToString(&e.format);
        let mut row = vec![
          &e.title,
          &e.author,
          &e.genre,
          &e.status,
          &formatString,
          &tags
        ];

        let idString = &e.id.to_string();
        if withId {
            row.push(idString);
        }

        table.add_row(row);
    }

    return table
}

fn createHeaderVec() -> Vec<String> {
    return vec![
        "author".to_string(),
        "format".to_string(),
        "genre".to_string(),
        "status".to_string(),
        "title".to_string()
    ];
}

fn createTableRowFromReadingEntry(re: &ReadingEntry) -> TableRow {
    let mut row = TableRow::new();
    row.insert(&"author".to_string(), &re.author);
    row.insert(&"format".to_string(), &formatEnumToString(&re.format));
    row.insert(&"genre".to_string(), &re.genre);
    row.insert(&"title".to_string(), &re.title);
    row.insert(&"status".to_string(), &re.status);

    return row;
}

fn initConfig() -> Config {
    let args = Cli::from_args();
    return Config {
        debug: args.debug,
        db_file_location: expandTilde(&args.db_file_location),
        withId: args.withId,
        query: args.q
    }
}

#[test]
fn test_expand_tilde() {
    assert_eq!(expandTilde(&"/foo/bar".to_string()), "/foo/bar".to_string());
    assert_eq!(expandTilde(&"~".to_string()), std::env::var("HOME").unwrap());
    assert_eq!(expandTilde(&"~/foo".to_string()), "/Users/tlongo/foo".to_string());
    assert_eq!(expandTilde(&"foo/bar".to_string()), "foo/bar".to_string());
}

fn expandTilde(dir: &String) -> String {
    let path = std::path::Path::new(dir);
    if path.has_root() == true {
        return dir.clone();
    }

    if dir.starts_with("~") == true {
        if (dir.len() == 1) {
            return std::env::var("HOME").unwrap();
        } else {
            let path_no_tilde: &str = &dir.as_str()[2..];
            return format!("{}/{}", std::env::var("HOME").unwrap(), &path_no_tilde);
        }
    }

    return dir.clone();
}

fn main() -> Result<()>{
    let args = Cli::from_args();

    let backend = SqliteBackend::new(&CONFIG.db_file_location)?;

    match args.cmd {
        Some(Command::Add) => {
            let re = add();
            backend.addEntry(&re)
        },
        Some(Command::Update{id}) => {
            let toUpdate = backend.getById(id)?;
            let updated = promptForUpdate(&toUpdate);
            backend.updateEntry(&updated);
        },
        Some(Command::Rm{id}) => {
            backend.deleteById(id);
        },
        None => {
            let entries = backend.getAllEntries()?;
            let columns = createHeaderVec();
            if CONFIG.query != "" {
                let modifier: Modifier = parse_query(&CONFIG.query, &columns);
                let filteredEntries = entries.into_iter()
                    .filter(|re| eval(&modifier, &mut createTableRowFromReadingEntry(&re)))
                    .collect();

                println!("{}", print_table(&filteredEntries, CONFIG.withId));
            } else {
                println!("{}", print_table(&entries, CONFIG.withId));
            }

        }
    }

    Ok(())
}
