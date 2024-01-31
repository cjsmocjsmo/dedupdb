use std::env;


pub mod envvars;
pub mod types;


fn main() {
    envvars::set_env_vars();
    let db_path = env::var("DUPS_DB");
    let conn = rusqlite::Connection::open(db_path.unwrap()).unwrap();
    let mut entryz_list = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM jpgs").unwrap();
    let mut rows = stmt.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let entry = types::Meta {
            imgid: row.get(0).unwrap(),
            imghash: row.get(1).unwrap(),
            imgpath: row.get(2).unwrap(),
        };
        entryz_list.push(entry);
    }
    for entry in entryz_list {
        println!("{:#?}", entry);
    }
}
