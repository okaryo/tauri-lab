use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const MIGRATIONS: &[(u32, &str)] = &[(1, include_str!("../migrations/001_initial.sql"))];

pub fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let path = database_path(app)?;

    Connection::open(path).map_err(|error| format!("Failed to open database: {error}"))
}

pub fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join("settings.json"))
}

pub fn init_database(app: &AppHandle) -> Result<(), String> {
    let mut connection = open_database(app)?;

    apply_migrations(&mut connection)?;
    write_schema_file(&connection)?;

    Ok(())
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    Ok(app_data_dir)
}

fn database_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join("data.sqlite"))
}

fn apply_migrations(connection: &mut Connection) -> Result<(), String> {
    let current_version = current_schema_version(connection)?;

    for (version, sql) in MIGRATIONS {
        if current_version >= *version {
            continue;
        }

        let transaction = connection
            .transaction()
            .map_err(|error| format!("Failed to start migration v{version}: {error}"))?;

        transaction
            .execute_batch(sql)
            .map_err(|error| format!("Failed to apply migration v{version}: {error}"))?;

        transaction
            .pragma_update(None, "user_version", version)
            .map_err(|error| format!("Failed to update schema version to v{version}: {error}"))?;

        transaction
            .commit()
            .map_err(|error| format!("Failed to commit migration v{version}: {error}"))?;
    }

    Ok(())
}

fn current_schema_version(connection: &Connection) -> Result<u32, String> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| format!("Failed to read schema version: {error}"))
}

fn write_schema_file(connection: &Connection) -> Result<(), String> {
    let schema = current_schema(connection)?;
    let path = schema_path();

    fs::write(path, schema).map_err(|error| format!("Failed to write schema file: {error}"))
}

fn schema_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schema.sql")
}

fn current_schema(connection: &Connection) -> Result<String, String> {
    let mut statement = connection
        .prepare(
            "
            SELECT sql
            FROM sqlite_schema
            WHERE sql IS NOT NULL
              AND name NOT LIKE 'sqlite_%'
              AND type IN ('table', 'index', 'trigger', 'view')
            ORDER BY
              CASE type
                WHEN 'table' THEN 1
                WHEN 'index' THEN 2
                WHEN 'trigger' THEN 3
                WHEN 'view' THEN 4
                ELSE 5
              END,
              name
            ",
        )
        .map_err(|error| format!("Failed to prepare schema query: {error}"))?;

    let statements = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| format!("Failed to query schema: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read schema: {error}"))?;

    Ok(format!("{};\n", statements.join(";\n\n")))
}
