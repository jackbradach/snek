use std::env;
use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct HighScore {
    name: String,
    date: String,
    points: u64,
    seconds: u64,
}

#[derive(Debug)]
struct HighScoreTable {
    conn: Connection,
}

impl HighScoreTable {
    pub fn add(&self, score: HighScore) {
        // println!("Adding {:?}", score);
        self.conn.execute(
            "INSERT INTO highscores (
                player_initials,
                date,
                points,
                seconds
            ) VALUES (
                ?1, ?2, ?3, ?4
            )",
            params![score.name, score.date, score.points, score.seconds ],
        ).unwrap();
    }


    pub fn get_highscore(&self) -> Option<HighScore> {
        let mut stmt =(self.conn.prepare(
            "SELECT player_initials, date, points, seconds
            FROM highscores
            WHERE points == (SELECT MAX(points) FROM highscores)"
        )).unwrap();
        // Desired behavior:
        // Try to get highest score
        // If multiple records are returned (same score), go for
        // shortest time.  If that somehow matches, winner is the
        // newest record.
        let mut highscores: Vec<HighScore> = Vec::new();
        let scores = stmt.query_map([], |row| {
            Ok(HighScore {
                name: row.get(0).unwrap(),
                date: row.get(1).unwrap(),
                points: row.get(2).unwrap(),
                seconds: row.get(3).unwrap(),
            })
        });
        
        // for s in scores {
        //     println!("{}", s);
        // }

        Some(scores.iter()[0])
        // self.conn.execute(
        //     "SELECT FROM highscores (
        //         player_initials,
        //         date,
        //         points,
        //         seconds
        //     ) WHERE (
        //         ?1, ?2, ?3, ?4
        //     )"
            
        // ).unwrap();
        // None
    }

    // Option?
    pub fn get_top_scores(&self) -> Option<Vec<HighScore>> {
        None
    }

    pub fn new(name: &str) -> HighScoreTable {
        HighScoreTable {
            conn: HighScoreTable::open_table(name),
        }
    }

    fn open_table(name: &str) -> Connection {
        let db_name = format!("{}.db", name);
        let conn = Connection::open(db_name).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS highscores (
                player_initials TEXT,
                date TEXT,
                points INTEGER,
                seconds INTEGER
            )",
            [],
        ).unwrap();
        conn
    }
}

// Impl display, have it get the top 10 records and display them.

#[cfg(test)] 
mod tests {
    use super::*;
    use rand::Rng;

    fn get_random_initials() -> String {
        let mut rng = rand::thread_rng();
        let first: char = rng.gen_range(b'A'..b'Z' + 1) as char;
        let middle: char = rng.gen_range(b'A'..b'Z' + 1) as char;
        let last: char = rng.gen_range(b'A'..b'Z' + 1) as char;
        format!("{}{}{}", first, middle, last)
    }

    fn get_random_date() -> String {
        // NOTE: I'm being lazy about day ranges per month here;
        // NOTE: the ficticious test players all have this superstition
        // NOTE: about playing games at the end of the month, it's weird.
        let mut rng = rand::thread_rng();
        let day: u32 = rng.gen_range(1..28);
        let month: u32 = rng.gen_range(1..13);
        let year: u32 = 2000 + rng.gen_range(0..100);
        format!("{}-{:02}-{:02}", year, month, day)
    }

    fn add_random_highscore(table: &HighScoreTable) {
        let mut rng = rand::thread_rng();
        let highscore = HighScore {
            name: get_random_initials(),
            date: get_random_date(),
            points: rng.gen_range(1..5000),
            seconds: rng.gen_range(1..3600),
        };
        table.add(highscore);
    }

    fn get_random_highscore_table(nentries: usize) -> HighScoreTable {
        let mut table = HighScoreTable::new("testo");
        // let mut dbfile = NamedTempFile::new().unwrap();
        for _ in 0..nentries {
            add_random_highscore(&table);
        }
        table
    }

    #[test]
    fn test_highscore_table() {
        let mut table = get_random_highscore_table(50);
        println!("{:?}", table);
        println!("highscore: {:?}", table.get_highscore());
    }

}

