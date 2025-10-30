use libsql::{params, Builder, Connection};
use std::{collections::HashMap, env, error::Error, path::PathBuf, time::SystemTime};

pub(crate) async fn init_db() -> Result<(), Box<dyn Error>> {
    let db = get_db().await?;
    if !(dose_table_exists("all_projects".to_string()).await?) {
        db.execute(
            "CREATE TABLE 'all_projects' (
                'id'	INTEGER NOT NULL UNIQUE,
                'name'	TEXT NOT NULL,
                'path'	TEXT NOT NULL UNIQUE,
                'last_access'	INTEGER NOT NULL DEFAULT 1757003813,
                PRIMARY KEY('id' AUTOINCREMENT)
            );",
            (),
        )
        .await?;
    }

    if !(dose_table_exists("open_projects".to_string()).await?) {
        db.execute(
            "CREATE TABLE 'open_projects' (
                'project_id'	INTEGER NOT NULL UNIQUE,
                'last_access'	INTEGER NOT NULL DEFAULT 1757003813,
                CONSTRAINT 'open_project_foreign_id' FOREIGN KEY('project_id') REFERENCES 'all_projects'('id')
            );",
            (),
        )
        .await?;
    }

    if !(dose_table_exists("ollama_setting".to_string()).await?) {
        db.execute(
            "CREATE TABLE 'ollama_setting' (
                'name'	TEXT NOT NULL UNIQUE,
                'value'	TEXT
            );",
            (),
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn get_recent_projects() -> Result<Vec<String>, Box<dyn Error>> {
    let db = get_db().await?;
    let mut result_data = vec![];
    let mut result = db
        .query(
            "SELECT id, name, path FROM all_projects ORDER BY last_access",
            (),
        )
        .await?;

    while let Some(data) = result.next().await? {
        result_data.push(data.get_str(2)?.to_string());
    }
    Ok(result_data)
}

pub(crate) async fn set_projects(name: String, path: String) -> Result<i64, Box<dyn Error>> {
    let db = get_db().await?;
    let mut dose_project_exists_row = db
        .query(
            "SELECT id, name, path FROM all_projects WHERE path=?1",
            [path.clone()],
        )
        .await?;
    if let Some(data) = dose_project_exists_row.next().await? {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        db.query(
            "UPDATE all_projects SET last_access=?1 WHERE path=?2",
            [now.as_secs().to_string(), path.clone()],
        )
        .await?;
        return Ok(data.get(0)?);
    } else {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        db.query(
            "INSERT INTO all_projects (name, path, last_access) VALUES (?1,?2,?3)",
            params![name, path.clone(), now.as_secs()],
        )
        .await?;
        let mut dose_project_exists_row = db
            .query(
                "SELECT id, name, path FROM all_projects WHERE path=?1",
                [path.clone()],
            )
            .await?;
        if let Some(data) = dose_project_exists_row.next().await? {
            return Ok(data.get(0)?);
        } else {
            panic!("Can't get created project")
        }
    }
}

pub(crate) async fn get_ollama_setting() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM ollama_setting;", ()).await?;
    let mut result_data = HashMap::new();
    while let Some(data) = result.next().await? {
        let setting_name = match data.get_value(0)?.as_text() {
            Some(e) => e.clone(),
            None => "".to_string(),
        };
        let setting_value = match data.get_value(1)?.as_text() {
            Some(e) => e.clone(),
            None => "".to_string(),
        };
        result_data.insert(setting_name, setting_value);
    }
    Ok(result_data)
}

pub(crate) async fn set_ollama_setting(
    setting_name: String,
    setting_value: String,
) -> Result<(), Box<dyn Error>> {
    let db = get_db().await?;
    let mut data_exists_result = db
        .query(
            "SELECT * FROM ollama_setting WHERE name=?1;",
            [setting_name.clone()],
        )
        .await?;
    if let Some(_data) = data_exists_result.next().await? {
        db.query(
            "UPDATE ollama_setting SET value=?1 WHERE name=?2;",
            [setting_value.clone(), setting_name.clone()],
        )
        .await?;
    } else {
        db.query(
            "INSERT INTO ollama_setting (name,value) VALUES (?1, ?2);",
            [setting_name.clone(), setting_value.clone()],
        )
        .await?;
    }
    Ok(())
}

async fn get_db() -> Result<Connection, Box<dyn Error>> {
    let project_list_path = "/home/cyber/Documents/tauri/project-man/config/config.db";

    let db = Builder::new_local(project_list_path).build().await.unwrap();
    let conn = db.connect()?;
    Ok(conn)
}

async fn dose_table_exists(table_name: String) -> Result<bool, Box<dyn Error>> {
    let table_exists;
    let db: Connection = get_db().await?;
    let mut result = db
        .query(
            "SELECT name FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [table_name.clone()],
        )
        .await?;

    if let Some(_data) = result.next().await? {
        table_exists = true;
    } else {
        table_exists = false;
    }

    Ok(table_exists)
}
