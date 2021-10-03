#![allow(dead_code)]

use doku::Document;
use serde::Deserialize;

#[derive(Deserialize, Document)]
struct Config {
    /// Database's engine
    db_engine: DbEngine,

    /// Database's host
    #[doku(example = "localhost")]
    db_host: String,

    /// Database's port
    #[doku(example = "5432")]
    db_port: usize,
}

#[derive(Deserialize, Document)]
enum DbEngine {
    #[serde(rename = "pgsql")]
    PostgreSQL,

    #[serde(rename = "mysql")]
    MySQL,
}

fn main() {
    println!("{}", doku::to_json::<Config>());
}
