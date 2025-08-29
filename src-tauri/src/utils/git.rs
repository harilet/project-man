use git2::{DiffFormat, DiffOptions, Repository, TreeWalkMode};
use std::{error::Error, path::Path};

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

pub(crate) fn get_file_diff(location: String, path: String) -> Result<String, Box<dyn Error>> {
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
            let content = str::from_utf8(l.content())
                .expect("Content is not utf-8")
                .to_string();
            diff_data.push(format!("{}:{}", l.origin(), content));
            true
        })?;

    Ok(diff_data.join(""))
}
