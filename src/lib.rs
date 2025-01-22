use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub mod prelude {
    pub use crate::{
        DatabaseConnection,
        ErrorTypePlayerHandler,
    };
}

pub struct BevyEasySharedDefinitionsPlugin {}

impl Plugin for BevyEasySharedDefinitionsPlugin {
    fn build(&self, app: &mut App) { // Builds automatically on .add_plugins() call
        app.insert_resource(BevyEasySuiteDebugToConsole::set(false));
    }
}

#[derive(Resource)]
pub struct BevyEasySuiteDebugToConsole {
    flag: bool,
}

impl BevyEasySuiteDebugToConsole {
    pub fn set(flag: bool) -> Self {
        BevyEasySuiteDebugToConsole {
            flag: flag,
        }
    }

    pub fn get(&self) -> bool {
        self.flag
    }
}

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

#[derive(Debug)]
pub enum ErrorTypePlayerHandler {
    AddPlayerFromDbToPartyFailed(String),
    DatabaseLockPoisoned,
    DBActionFailed(String),
    DBQueryFailed(String),
    DBQueryMappingFailed(String),
    IndexOutOfBounds(String),
    LockFailed(String),
    PartyActionFailed(String),
    PartySizeAtSetLimit,
    PartySizeGreaterThanSetLimit,
    PlayerAiCallFailed(String),
    PlayerLocalCallFailed(String),
    PlayerMainCallFailed(String),
    PlayerRemoteCallFailed(String),
    PluginDataRetreivalFailed(String),
    PoisonErrorBox(String),
    UuidParsingFailed(String),
    VarErrorNotPresent,
    VarErrorNotUnicode(String),
}