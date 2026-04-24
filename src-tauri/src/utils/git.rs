use git2::{DiffFormat, DiffOptions, Repository};
use std::{error::Error, path::Path};
use crate::utils::ollama_tool;

#[derive(Clone, serde::Serialize, Debug)]
struct ChangeLine {
    from_no: String,
    to_no: String,
    content: String,
    change_type: String,
}

pub(crate) fn get_current_branch_name(location: String) -> Result<String, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let head = repo.head()?;
    Ok(head.shorthand().expect("Empty Branch Name").to_string())
}

fn get_repo(location: String) -> Result<Repository, Box<dyn Error>> {
    return Ok(Repository::open(location)?);
}

pub(crate) fn get_staged_files(location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    let old_tree = repo.head()?.peel_to_tree()?;

    let staged_diff =
        repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?;

    for diff in staged_diff.deltas().into_iter() {
        path_list.push(
            diff.new_file()
                .path()
                .expect("Empty File Path")
                .to_str()
                .expect("Cannot Convert Path To &str")
                .to_string(),
        );
    }
    Ok(path_list)
}

pub(crate) fn get_unstaged_files(location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(true)
        .minimal(true)
        .include_ignored(false)
        .include_untracked(true)
        .ignore_whitespace_eol(false);
    let unstaged_diff = repo.diff_index_to_workdir(Some(&repo.index()?), Some(&mut diff_opts))?;
    for diff in unstaged_diff.deltas().into_iter() {
        path_list.push(match diff.new_file().path() {
            Some(path) => match path.to_str() {
                Some(path_str) => path_str.to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        });
    }
    Ok(path_list)
}

pub(crate) fn get_file_diff(
    location: String,
    path: String,
    is_unified: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(true)
        .minimal(true)
        .include_ignored(false)
        .include_untracked(false)
        .ignore_whitespace_eol(false)
        .pathspec(path.clone());
    let old_tree = repo.head()?.peel_to_tree()?;

    let mut diff_data: Vec<String> = vec![];
    let diff =
        repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?;

    if is_unified {
        diff.print(DiffFormat::Patch, |_d, _, line| {
            let c_line = format!(
                "{}{}",
                line.origin().to_string(),
                String::from_utf8_lossy(line.content())
            );
            diff_data.push(c_line);
            true
        })?;
        return Ok(diff_data);
    }

    diff.print(
        DiffFormat::Patch,
        |_d, _h, l| match format_change_line_item(l, _d) {
            Ok(data) => {
                diff_data.push(data);
                true
            }
            Err(_) => false,
        },
    )?;

    return Ok(diff_data);
}

pub(crate) fn get_unstaged_file_diff(
    location: String,
    path: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(false)
        .minimal(false)
        .include_ignored(false)
        .include_untracked(false)
        .ignore_whitespace_eol(false)
        .pathspec(path.clone());

    let mut diff_data: Vec<String> = vec![];

    repo.diff_index_to_workdir(Some(&repo.index()?), Some(&mut diff_opts))?
        .print(
            DiffFormat::Patch,
            |_d, _h, l| match format_change_line_item(l, _d) {
                Ok(data) => {
                    diff_data.push(data);
                    true
                }
                Err(_) => false,
            },
        )?;
    Ok(diff_data)
}

fn format_change_line_item(
    l: git2::DiffLine,
    d: git2::DiffDelta,
) -> Result<String, Box<dyn Error>> {
    let mut temp_data = ChangeLine {
        content: "".to_string(),
        to_no: "".to_string(),
        from_no: "".to_string(),
        change_type: "".to_string(),
    };

    let mut content = str::from_utf8(l.content()).unwrap().to_string();

    temp_data.change_type = l.origin().to_string();

    match l.old_lineno() {
        Some(num) => {
            temp_data.from_no = format!("{}", num);
        }
        None => {
            temp_data.from_no = format!("");
        }
    }

    match l.new_lineno() {
        Some(num) => {
            temp_data.to_no = format!("{}", num);
        }
        None => {
            temp_data.to_no = format!("",);
        }
    }

    if temp_data.change_type == "F" {
        match d.status() {
            git2::Delta::Modified => {
                temp_data.change_type = "M".to_owned();
            }
            git2::Delta::Added => {
                temp_data.change_type = "A".to_owned();
            }
            git2::Delta::Deleted => {
                temp_data.change_type = "D".to_owned();
            }
            git2::Delta::Renamed => {
                temp_data.change_type = "R".to_owned();
            }
            _ => {}
        };
        content = d.new_file().path().unwrap().to_str().unwrap().to_string();
    }

    temp_data.content = content;
    Ok(serde_json::to_string(&temp_data)?)
}

pub(crate) fn add_file_index(location: String, path: String) -> Result<(), Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    let mut index = repo.index()?;
    index.add_path(Path::new(&path))?;
    index.write()?;
    Ok(())
}

pub(crate) fn remove_file_index(location: String, path: String) -> Result<(), Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    repo.reset_default(Some(commit.as_object()), &[path])?;
    Ok(())
}

pub(crate) fn get_project_tree(location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut outputs = vec![];
    let mut output = String::new();
    let depth = 1;
    let base = std::path::Path::new(&location);
    ollama_tool::collect_dir_entries(base, base, 0, depth, &mut output);
    outputs.push(output);

    Ok(outputs)
}

pub(crate) fn get_all_staged_diff(location: String) -> Result<String, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut diff_opts = DiffOptions::new();
    let old_tree = repo.head()?.peel_to_tree()?;
    let mut diff_data: Vec<String> = vec![];

    let diff =
        repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?;

    diff.print(DiffFormat::Patch, |_d, _, line| {
        let c_line = format!(
            "{}{}",
            line.origin().to_string(),
            String::from_utf8_lossy(line.content())
        );
        diff_data.push(c_line);
        true
    })?;

    Ok(diff_data.join("\n"))
}
