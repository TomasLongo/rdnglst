use rusqlite::{Connection, Result};
use rusqlite::{NO_PARAMS, MappedRows, types::FromSql, types::FromSqlResult, types::ValueRef};

#[derive(Debug)]
pub struct ReadingEntry {
    pub id: i64,
    pub author: String,
    pub title: String,
    pub genre: String,
    pub format: Format,
    pub status: String,
    pub tags: Vec<String>
}

#[derive(Debug)]
pub enum Format {
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

pub fn stringToFormatEnum(toConvert: &str) -> Format {
    let lc = toConvert.to_lowercase();
    if lc == "book" {
        return Format::Book;
    } else if lc == "kindle" {
        return Format::Kindle
    } else {
        return Format::Book
    }
}

pub fn formatEnumToString(f: &Format) -> String {
    match f {
        Format::Book => return String::from("book"),
        Format::Kindle => return String::from("kindle")
    };
}

pub trait Backend {
    fn getById(&self, id: i64) -> Result<ReadingEntry>;
    fn updateEntry(&self, toUpdate: &ReadingEntry);
    fn addEntry(&self, e: &ReadingEntry);
    fn getAllEntries(&self) -> Result<Vec<ReadingEntry>>;
}

pub struct SqliteBackend {
    conn: Connection
}

impl SqliteBackend {
    pub fn new(db_name: &str) -> Result<SqliteBackend> {
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

        return Ok(SqliteBackend {
            conn
        })
    }
}

impl Backend for SqliteBackend {
    fn getById(&self, id: i64) -> Result<ReadingEntry> {
        let mut stmt = self.conn.prepare("SELECT id, title, author, genre, format, tags, status FROM reading_entries where id = ?1")?;

        let entry = stmt.query_row(&[id], |row| {
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

        Ok(entry)
    }
    
    fn updateEntry(&self, toUpdate: &ReadingEntry) {
        let insertString = "update reading_entries set title = ?1, author = ?2, genre = ?3, format = ?4, tags = ?5, status = ?6 where id = ?7;";
        self.conn.execute(insertString, 
                          &[&toUpdate.title, &toUpdate.author, &toUpdate.genre, &formatEnumToString(&toUpdate.format), &toUpdate.tags.join(" "), &toUpdate.status, &toUpdate.id.to_string()]);
    }

    fn addEntry(&self, re: &ReadingEntry) {
        let insertString = "insert into reading_entries (title, author, genre, format, tags, status) values (?1, ?2, ?3, ?4, ?5, ?6);";
        self.conn.execute(insertString, &[&re.title, &re.author, &re.genre, &formatEnumToString(&re.format), &re.tags.join(" "), &re.status]);
    }

    fn getAllEntries(&self) -> Result<Vec<ReadingEntry>> {
        let mut stmt = self.conn.prepare("SELECT id, title, author, genre, format, tags, status FROM reading_entries")?;

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
}
