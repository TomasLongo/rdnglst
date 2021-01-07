use structopt::StructOpt;
use dialoguer::{Input, theme::ColorfulTheme};
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

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

#[derive(Debug)]
struct ReadingEntry {
    author: String,
    title: String,
    genre: String,
    format: Format,
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
    let author: String = create_promt_for(&String::from("Author"));
    let title: String = create_promt_for(&String::from("Title"));
    let format: String = create_promt_for(&String::from("Format"));
    let genre: String = create_promt_for(&String::from("Genre"));
    let tags: String = create_promt_for(&String::from("Tags"));

    let splits = tags.split(" ");

    // gibt es hierfuer nicht schon was in der std lib?
    let mut vec: Vec<String> = Vec::new();
    for s in splits {
        vec.push(String::from(s));
    }

    let re = ReadingEntry{
        author: author,
        title: title,
        format: stringToFormatEnum(&format),
        genre: genre,
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

fn init_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        " create table if not exists reading_entries (
            id integer primary key autoincrement,
            title text,
            author text,
            genre text,
            format text,
            tags text,
            created_at timestamp default current_timestamp,
            updated_at timestamp default current_timestamp
            );
        ", NO_PARAMS,)?;

    return Ok(conn);
}

fn insertReadingEntry(re: &ReadingEntry, conn: &Connection) {
    let insertString = "insert into reading_entries (title, author, genre, format, tags) values (?1, ?2, ?3, ?4, ?5);";
    conn.execute(insertString, &[&re.title, &re.author, &re.genre, &formatEnumToString(&re.format), &re.tags.join(" ")]);
}

fn main() -> Result<()>{
    let args = Cli::from_args();

    let dbConn = init_db("./readinglist.db")?;
    match args.cmd {
        Some(Command::Add) => {
            let re = add();
            insertReadingEntry(&re, &dbConn)
        },
        None => println!("print nice table with reading list here")
    }

    Ok(())
}
