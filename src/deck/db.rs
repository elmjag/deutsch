use rusqlite::{fallible_iterator::FallibleIterator, Connection, Result};
use std::ops::Range;

use super::nouns::NOUNS;

const DB_SCHEMA: &str = "
    create table if not exists nouns(
        noun_index integer integer check(noun_index >= 0) primary key,
        level integer default 0)
    ";

const GET_NOUNS_QUERY: &str = "
    select noun_index, level from nouns
        order by level, random()
        limit ?1
    ";

const UPDATE_LEVEL_QUERY: &str = "
    update nouns
        set level = ?1
        where noun_index = ?2
    ";

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

fn get_first_missing_noun_index(conn: &Connection) -> Result<u32> {
    let res: Result<u32> = conn.query_row(
        "select noun_index from nouns order by -noun_index limit 1",
        [],
        |row| row.get(0),
    );

    match res {
        Ok(val) => Ok(val + 1),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        err => err,
    }
}

fn add_indices_to_db(conn: &Connection, indices: Range<u32>) -> Result<()> {
    let mut query = String::from("insert into nouns(noun_index) values ");
    let last_idx = indices.end - 1;

    for idx in indices {
        let q = String::from(format!("({idx})"));
        query.push_str(&q);
        if idx != last_idx {
            query.push(',');
        }
    }

    conn.execute(&query, ())?;

    Ok(())
}

fn maybe_add_nouns(conn: &Connection) -> Result<()> {
    let start_idx = get_first_missing_noun_index(conn)?;
    let end_idx = NOUNS.len() as u32;

    if start_idx == end_idx {
        /* no new indices needs to be added, we are done */
        return Ok(());
    }

    add_indices_to_db(conn, start_idx..end_idx)?;

    Ok(())
}

fn setup_db(db_file_path: &str) -> Result<Connection> {
    /* connect to the database */
    let conn = Connection::open(db_file_path)?;

    /* maybe create set-up database schema */
    conn.execute(DB_SCHEMA, ())?;

    /* maybe add missing noun indices */
    maybe_add_nouns(&conn)?;

    Ok(conn)
}

impl Database {
    pub fn connect(db_file_path: &str) -> Result<Database> {
        Ok(Database {
            connection: setup_db(db_file_path)?,
        })
    }

    pub fn get_nouns(&self, num_nouns: usize) -> Result<Vec<(usize, u32)>> {
        let mut stmt = self.connection.prepare(GET_NOUNS_QUERY)?;
        let rows = stmt.query((num_nouns,))?;

        rows.map(|r| {
            let idx = r.get(0)?;
            let level = r.get(1)?;
            Ok((idx, level))
        })
        .collect()
    }

    pub fn update_level(&self, noun_index: usize, level: u32) -> Result<()> {
        self.connection
            .execute(UPDATE_LEVEL_QUERY, (level, noun_index))?;
        Ok(())
    }
}
