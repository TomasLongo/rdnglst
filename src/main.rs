use structopt::StructOpt;
use dialoguer::{Input, theme::ColorfulTheme};
use rusqlite::{Connection, Result};

// Insight: Damit der Compiler Trait-Methoden an einer Struct aufrufen kann, muss das Trait
// mit importiert werden.
use readinglist::{Backend, SqliteBackend, ReadingEntry, formatEnumToString, stringToFormatEnum};

use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    #[structopt(short="i", long = "id")]
    withId: bool
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

fn main() -> Result<()>{
    let args = Cli::from_args();

    let dbLocation = "/Users/tlongo/projects/readinglist/reading-list/readinglist.db";

    let backend = SqliteBackend::new(dbLocation)?;
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

            println!("{}", print_table(&entries, args.withId));
        }
    }

    Ok(())
}
