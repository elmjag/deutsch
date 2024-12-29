use std::io;

mod db;
use db::Database;

fn ask_noun(db: &Database) {
    let noun = db.get_noun();
    let mut misstake = false;

    loop {
        println!("___ {}?", noun.word);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("I/O error");

        if guess.trim().to_lowercase() == noun.article {
            println!("correct");
            break;
        }

        misstake = true;
        println!("wrong, try again");
    }

    match misstake {
        true => db.decrease_level(&noun),
        false => db.increase_level(&noun),
    }
}

fn main() {
    let db = Database::init();
    loop {
        ask_noun(&db);
    }
}
