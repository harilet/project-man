use git2::{DiffFormat, DiffOptions, Repository, TreeWalkMode};
use std::{error::Error, path::Path};

#[derive(Clone, serde::Serialize, Debug)]
struct ChangeLine {
    from_no: String,
    to_no: String,
    content: String,
    change_type: String,
}

fn get_current_branch_name(repo: &Repository) -> Result<String, Box<dyn Error>> {
    let head = repo.head()?;
    Ok(head.shorthand().expect("Empty Branch Name").to_string())
}

fn get_repo(location: String) -> Result<Repository, Box<dyn Error>> {
    return Ok(Repository::open(location)?);
}

pub(crate) fn get_project_struture(location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let repo: Repository;
    match get_repo(location) {
        Ok(t_repo) => {
            repo = t_repo;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    let mut list_of_files = vec![];

    let rev = get_current_branch_name(&repo)?;
    let obj = repo.revparse_single(&rev)?;
    let tree = obj.peel_to_tree()?;

    tree.walk(TreeWalkMode::PreOrder, |path, file| {
        let file = format!("{}{}", path, file.name().expect("File Name Empty"));
        if !(Path::new(&file).is_dir()) {
            list_of_files.push(file);
        }
        git2::TreeWalkResult::Ok
    })?;

    Ok(list_of_files)
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

pub(crate) fn get_file_diff(location: String, path: String) -> Result<Vec<String>, Box<dyn Error>> {
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
    repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?
        .print(DiffFormat::Patch, |_d, _h, l| {
            diff_data.push(format_change_line_item(l, _d).unwrap());
            true
        })?;

    Ok(diff_data)
}

fn format_change_line_item(l: git2::DiffLine, d: git2::DiffDelta) -> Result<String, Box<dyn Error>> {
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
