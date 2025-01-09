use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

#[derive(Resource)]
pub struct DatabaseConnection {
    pub conn: Arc<Mutex<Connection>>,
}

impl DatabaseConnection {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).expect("Failed to open SQLite database");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}