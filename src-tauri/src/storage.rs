use std::path::PathBuf;

use log::info;
use rusqlite::{Connection, Error};

const DB_FILE: &str = "datadriver.db";

fn open_connection() -> Result<Connection, Error> {
    let database = PathBuf::from(&crate::app::APP.lock().app_dir)
        .join(DB_FILE);
    let conn = Connection::open(database)?;
    Ok(conn)
}

pub fn exec<F, T>(func: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Connection) -> Result<T, Error>,
{
    match open_connection() {
        Ok(mut conn) => func(&mut conn),
        Err(e) => Err(e),
    }
}

fn init_table() {
    let res = exec(|conn| {
        // Initialize the article table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS `dd_connector`
                (
                    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                    `name` VARCHAR(200) NOT NULL,
                    `connector` VARCHAR(200) NOT NULL,
                    `description` VARCHAR(2000) NOT NULL DEFAULT 'No description',
                    `host` VARCHAR(2000) NOT NULL,
                    `port` INTEGER NOT NULL DEFAULT 80,
                    `username` VARCHAR(2000) NOT NULL,
                    `password` VARCHAR(2000) NOT NULL,
                    `database` VARCHAR(2000) NOT NULL,
                    `available` INTEGER NOT NULL DEFAULT 0,
                    `create_time` timestamp DEFAULT current_timestamp,
                    `update_time` timestamp DEFAULT current_timestamp
                )",
            [],
        )?;
        info!("Create table dd_connector successfully");
        Ok(true)
    });
    info!("Initialize tables {:?}", res.unwrap());
}

pub fn check() {
    init_table();
}
