use std::env;
use std::process;

pub mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a database path as an argument");
        process::exit(1);
    }
    let db_path = &args[1];
    
    let conn = rusqlite::Connection::open(db_path).unwrap();
    let mut entryz_list = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM jpgs").unwrap();
    let mut rows = stmt.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let entry = types::Meta {
            imgid: row.get(1).unwrap(),
            imghash: row.get(2).unwrap(),
            imgpath: row.get(3).unwrap(),
        };
        entryz_list.push(entry);
    }

    for entry in entryz_list {
        let mut scan_list = Vec::new();
        let mut stmt2 = conn.prepare("SELECT * FROM jpgs WHERE imghash = ?").unwrap();
        let mut rows2 = stmt2.query([entry.imghash]).unwrap();
        while let Some(row) = rows2.next().unwrap() {
            let entry = types::Meta {
                imgid: row.get(1).unwrap(),
                imghash: row.get(2).unwrap(),
                imgpath: row.get(3).unwrap(),
            };
            scan_list.push(entry);
        };

        if scan_list.len() > 1 {
            println!("Duplicate found: {:#?}", scan_list[0]);
        }
    }
}