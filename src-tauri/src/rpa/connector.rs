use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::response::Response;
use crate::storage::exec;

#[derive(Serialize, Deserialize)]
pub struct Connector {
    id: i32,
    name: String,
    connector: String,
    description: String,
    host: String,
    port: i32,
    username: String,
    password: String,
    database: String,
    available: i32,
    create_time: String,
    update_time: String,
}

/**
 * Get all connectors
 **/
#[tauri::command]
pub fn get_connector() -> Response<Vec<Connector>> {
    match exec(|conn| {
        let mut stmt = conn.prepare("SELECT * FROM `dd_connector`")?;
        let mut res = stmt.query(params![])?;
        let mut vec = vec![];
        while let Some(row) = res.next()? {
            vec.push(Connector {
                id: row.get("id")?,
                name: row.get("name")?,
                connector: row.get("connector")?,
                description: row.get("description")?,
                host: row.get("host")?,
                port: row.get("port")?,
                username: row.get("username")?,
                password: row.get("password")?,
                database: row.get("database")?,
                available: row.get("available")?,
                create_time: row.get("create_time")?,
                update_time: row.get("update_time")?,
            })
        }
        Ok(vec)
    })
    {
        Ok(records) => Response::new(200, records, "Successful！".to_string()),
        Err(_err) => Response::new(500, Vec::new(), _err.to_string()),
    }
}
