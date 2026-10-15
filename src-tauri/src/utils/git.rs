use git2::{DiffFormat, DiffOptions, Repository};
use std::{error::Error, path::Path};

#[derive(Clone, serde::Serialize, Debug)]
struct ChangeLine {
    from_no: String,
    to_no: String,
    content: String,
    change_type: String,
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub short_id: String,
    pub summary: String,
    pub author: String,
    pub time: i64,
}

pub(crate) fn get_current_branch_name(location: String) -> Result<String, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let head = repo.head()?;
    Ok(head.shorthand().expect("Empty Branch Name").to_string())
}

fn get_repo(location: String) -> Result<Repository, Box<dyn Error>> {
    Ok(Repository::open(location)?)
}

pub(crate) fn get_staged_files(location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    let old_tree = repo.head()?.peel_to_tree()?;

    let staged_diff =
        repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?;

    for diff in staged_diff.deltas() {
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
    let repo = get_repo(location)?;

    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(true)
        .minimal(true)
        .include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .ignore_whitespace_eol(false);
    let unstaged_diff = repo.diff_index_to_workdir(Some(&repo.index()?), Some(&mut diff_opts))?;
    for diff in unstaged_diff.deltas() {
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
    let repo = get_repo(location)?;

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
                line.origin(),
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

    Ok(diff_data)
}

pub(crate) fn get_unstaged_file_diff(
    location: String,
    path: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(false)
        .minimal(false)
        .include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .show_untracked_content(true)
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
            temp_data.from_no = String::new();
        }
    }

    match l.new_lineno() {
        Some(num) => {
            temp_data.to_no = format!("{}", num);
        }
        None => {
            temp_data.to_no = String::new();
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
    let repo = get_repo(location)?;
    let mut index = repo.index()?;
    index.add_path(Path::new(&path))?;
    index.write()?;
    Ok(())
}

pub(crate) fn remove_file_index(location: String, path: String) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location)?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    repo.reset_default(Some(commit.as_object()), &[path])?;
    Ok(())
}

pub(crate) fn get_all_staged_diff(location: String) -> Result<String, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut diff_opts = DiffOptions::new();
    let old_tree = repo.head()?.peel_to_tree()?;
    let mut diff_data: Vec<String> = vec![];

    let diff =
        repo.diff_tree_to_index(Some(&old_tree), Some(&repo.index()?), Some(&mut diff_opts))?;

    diff.print(DiffFormat::Patch, |_d, _, line| {
        let c_line = format!(
            "{}{}",
            line.origin(),
            String::from_utf8_lossy(line.content())
        );
        diff_data.push(c_line);
        true
    })?;

    Ok(diff_data.join("\n"))
}

fn get_signature(repo: &Repository) -> Result<git2::Signature<'static>, Box<dyn Error>> {
    match repo.signature() {
        Ok(sig) => Ok(sig.to_owned()),
        Err(_) => Ok(git2::Signature::now("project-man", "project-man@localhost")?),
    }
}

pub(crate) fn commit(location: String, message: String) -> Result<String, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = get_signature(&repo)?;

    let parent = match repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };
    let parents: Vec<&git2::Commit> = parent.iter().collect();

    let oid = repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &parents)?;
    Ok(oid.to_string())
}

pub(crate) fn amend_commit(location: String, message: String) -> Result<String, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = get_signature(&repo)?;

    let head_commit = repo.head()?.peel_to_commit()?;
    let message = if message.trim().is_empty() {
        head_commit.message().unwrap_or("").to_string()
    } else {
        message
    };

    let oid = head_commit.amend(
        Some("HEAD"),
        Some(&sig),
        Some(&sig),
        None,
        Some(&message),
        Some(&tree),
    )?;
    Ok(oid.to_string())
}

pub(crate) fn get_branches(location: String) -> Result<Vec<BranchInfo>, Box<dyn Error>> {
    let repo = get_repo(location)?;
    let head_name = current_branch_name(&repo);

    let mut branches: Vec<BranchInfo> = vec![];
    for item in repo.branches(Some(git2::BranchType::Local))? {
        let (branch, _) = item?;
        let name = match branch.name()? {
            Some(name) => name.to_string(),
            None => continue,
        };
        branches.push(BranchInfo {
            is_head: Some(name.clone()) == head_name,
            is_remote: false,
            name,
        });
    }

    branches.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(branches)
}

fn current_branch_name(repo: &Repository) -> Option<String> {
    repo.head().ok().and_then(|head| {
        if head.is_branch() {
            head.shorthand().map(|name| name.to_string())
        } else {
            None
        }
    })
}

pub(crate) fn create_branch(
    location: String,
    name: String,
    checkout: bool,
) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location.clone())?;
    {
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        repo.branch(&name, &commit, false)?;
    }
    drop(repo);

    if checkout {
        checkout_branch(location, name)?;
    }
    Ok(())
}

pub(crate) fn checkout_branch(location: String, name: String) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location)?;
    let (object, reference) = repo.revparse_ext(&name)?;

    repo.checkout_tree(&object, None)?;
    match reference {
        Some(reference) => {
            let ref_name = reference
                .name()
                .ok_or("Reference has an invalid name")?
                .to_string();
            repo.set_head(&ref_name)?;
        }
        None => repo.set_head_detached(object.id())?,
    }
    Ok(())
}

pub(crate) fn rename_branch(
    location: String,
    old_name: String,
    new_name: String,
) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location)?;
    let mut branch = repo.find_branch(&old_name, git2::BranchType::Local)?;
    branch.rename(&new_name, false)?;
    Ok(())
}

pub(crate) fn delete_branch(location: String, name: String) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location)?;

    if current_branch_name(&repo) == Some(name.clone()) {
        return Err("Cannot delete the branch you are currently on".into());
    }

    let mut branch = repo.find_branch(&name, git2::BranchType::Local)?;
    branch.delete()?;
    Ok(())
}

pub(crate) fn get_commit_log(
    location: String,
    limit: usize,
) -> Result<Vec<CommitInfo>, Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut revwalk = repo.revwalk()?;
    if revwalk.push_head().is_err() {
        return Ok(vec![]);
    }

    let mut commits: Vec<CommitInfo> = vec![];
    for (index, oid) in revwalk.enumerate() {
        if index >= limit {
            break;
        }
        let oid = match oid {
            Ok(oid) => oid,
            Err(_) => continue,
        };
        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(_) => continue,
        };
        let id = oid.to_string();
        commits.push(CommitInfo {
            short_id: id.chars().take(7).collect(),
            id,
            summary: commit.summary().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("").to_string(),
            time: commit.time().seconds(),
        });
    }

    Ok(commits)
}

pub(crate) fn discard_file_changes(location: String, path: String) -> Result<(), Box<dyn Error>> {
    let repo = get_repo(location)?;

    let mut checkout_builder = git2::build::CheckoutBuilder::new();
    checkout_builder.path(&path).force();
    repo.checkout_head(Some(&mut checkout_builder))?;
    Ok(())
}

fn run_git(location: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(location)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() { stderr } else { stdout })
    } else {
        let message = if stderr.is_empty() { stdout } else { stderr };
        Err(message.into())
    }
}

pub(crate) fn fetch(location: String, remote: String) -> Result<String, Box<dyn Error>> {
    run_git(&location, &["fetch", "--prune", &remote])
}

pub(crate) fn push(
    location: String,
    remote: String,
    branch: String,
) -> Result<String, Box<dyn Error>> {
    run_git(&location, &["push", "--set-upstream", &remote, &branch])
}

pub(crate) fn pull(
    location: String,
    remote: String,
    branch: String,
) -> Result<String, Box<dyn Error>> {
    run_git(&location, &["pull", "--ff-only", &remote, &branch])
}
