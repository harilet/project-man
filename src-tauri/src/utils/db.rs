use libsql::{params, Builder, Connection};
use std::{env, error::Error, path::PathBuf};

pub(crate) async fn init_db() -> Result<(), Box<dyn Error>> {
    let db = get_db().await?;
    if !(dose_table_exists("all_projects".to_string()).await?) {
        db.execute(
            "CREATE TABLE 'all_projects' (
                'id'	INTEGER NOT NULL UNIQUE,
                'name'	TEXT NOT NULL,
                'path'	INTEGER NOT NULL UNIQUE,
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
	CONSTRAINT 'open_project_foreign_id' FOREIGN KEY('project_id') REFERENCES 'all_projects'('id')
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
        .query("SELECT id, name, path FROM all_projects", ())
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
        return Ok(data.get(0)?);
    } else {
        db.query(
            "INSERT INTO all_projects (name, path) VALUES (?1,?2)",
            params![name, path.clone()],
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
        }else{
            panic!("Can't get created project")
        }
    }
}

async fn get_db() -> Result<Connection, Box<dyn Error>> {
    let exe_path: PathBuf = get_exe_dir()?;
    let project_list_path = exe_path.join("config\\config.db");

    let db = Builder::new_local(project_list_path.to_str().unwrap())
        .build()
        .await
        .unwrap();
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

fn get_exe_dir() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    return Ok(exe_path
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf());
}
