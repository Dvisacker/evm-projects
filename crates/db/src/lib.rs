use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod models;
pub mod queries;
pub mod schema;

// pub const MIGRATIONS: EmbeddedMigrations =
//     embed_migrations!("migrations/2024-08-29-000000_create_pools_with_constraints");

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn run_migrations(
//     conn: &mut SqliteConnection,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//     conn.run_pending_migrations(MIGRATIONS)?;
//     Ok(())
// }
