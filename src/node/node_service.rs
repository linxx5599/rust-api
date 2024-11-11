#[path = "../sql.rs"]
mod sql;

// use chrono::{DateTime, Utc};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

#[derive(Serialize, Deserialize)]
struct RowData {
    uuid: String,
    name: String,
    password: String,
    // created_at: DateTime<Utc>,
    // updated_at: DateTime<Utc>,
}

pub fn get_node() -> Value {
    let config = sql::DatabaseConfig::new().unwrap();
    let pool = config.create_pool().unwrap();
    let mut conn: PooledConn = pool.get_conn().unwrap();
    let rows: Result<Vec<Row>, Error> = Ok(conn.exec("SELECT * FROM user", ()).unwrap());
    let result: Vec<Row> = rows.ok().unwrap();
    let result: Vec<_> = result
        .iter()
        .map(|row: &Row| {
            // let created_at: String = row.get(2).unwrap();
            // let updated_at: String = row.get("updated_at").unwrap();
            RowData {
                uuid: row.get("uuid").unwrap(),
                name: row.get("name").unwrap(),
                password: row.get("password").unwrap(),
                // created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
                // updated_at: DateTime::from_timestamp(updated_at, 0).unwrap(),
            }
        })
        .collect();
    return to_value(result).unwrap();
}
