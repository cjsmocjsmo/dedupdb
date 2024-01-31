use std::env;


pub mod envvars;
pub mod types;


fn main() {
    envvars::set_env_vars();
    let db_path = env::var("DUPS_DB");
    let conn = rusqlite::Connection::open(db_path.unwrap()).unwrap();
    let mut entryz_list = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM jpgs").unwrap();
    let entryz = stmt.query_map([], |row| {
        Ok(types::Meta {
            imgid: row.get(0)?,
            imghash: row.get(1)?,
            imgpath: row.get(2)?,
        })
    }).unwrap();
    for entry in entryz {
        entryz_list.push(entry.unwrap());
    }

    for e in entryz_list {
        println!("{:?}", e);
    }
}
