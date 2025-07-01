use sea_orm::*;

pub mod entities;
pub mod migrator;

const DATABASE_URL: &str = "sqlite:./sqlite.db";
const DB_NAME: &str = "bakeries_db";

pub async fn run() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    match db.get_database_backend() {
        DatabaseBackend::MySql => {
            create_db(db, format!("CREATE DATABASE IF NOT EXISTS '{}';", DB_NAME)).await
        }
        DatabaseBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            create_db(
                db,
                format!("CREATE DATABASE IF NOT EXISTS \"{}\";", DB_NAME),
            )
            .await
        }
        DatabaseBackend::Sqlite => Ok(db),
    }
}

async fn create_db(db: DatabaseConnection, sql_query: String) -> Result<DatabaseConnection, DbErr> {
    db.execute(Statement::from_string(db.get_database_backend(), sql_query))
        .await?;
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    Ok(Database::connect(&url).await?)
}
