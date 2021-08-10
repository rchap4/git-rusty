// Copyright RChapman 2021
// This file is part of git-rusty, Rust examples for Git Operations.

// git-rusty is free software: you can redistribute it and/or modify
// it under the terms of the Affero GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// git-rusty is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero GNU General Public License for more details.

// You should have received a copy of the Affero GNU General Public License
// along with git-rusty.  If not, see <https://www.gnu.org/licenses/>.

// As a learning exericse there are a few unused functions
// and dead code paths.  These have been allowed and annotated
// as needed

#![allow(dead_code)]
use git2::{Branch, Config, Cred, Error, Remote, RemoteCallbacks, Repository};
use std::path::Path;

// Rename me as we use this call back for clone and push
fn clone_callback(config: Config) -> Result<RemoteCallbacks<'static>, Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        Cred::credential_helper(&config, _url, username_from_url)
    });
    Ok(callbacks)
}

#[allow(unused)]
fn get_branches(repo: &Repository) -> Result<Vec<Branch>, Error> {
    let mut branches_result = vec![];
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    for branch in branches {
        branches_result.push(branch?.0);
    }
    Ok(branches_result)
}

#[allow(unused)]
fn get_branch_names(repo: &Repository) -> Result<Vec<String>, Error> {
    let mut branch_names = vec![];
    let branches = get_branches(repo)?;
    for branch in branches {
        if let Some(n) = branch.name()? {
            branch_names.push(n.to_string());
        }
    }
    Ok(branch_names)
}

#[allow(unused)]
fn get_ref_spec(branch_names: Vec<String>) -> String {
    let ref_spec = branch_names
        .into_iter()
        .filter(|n| n == "main" || n == "master");
    ref_spec.collect()
}
fn get_head(repo: &Repository) -> Result<String, Error> {
    let branches = repo.branches(None)?;
    branches.fold(
        Err(git2::Error::new(
            git2::ErrorCode::GenericError,
            git2::ErrorClass::Repository,
            "Couldn't get branches",
        )),
        |acc, branch| {
            let b = branch?;
            if b.0.is_head() {
                let name = b.0.name()?;
                return match name {
                    Some(n) => Ok(n.to_string()),
                    None => return acc,
                };
            }
            acc
        },
    )
}

pub fn clone_repo(
    url: &str,
    clone_bare: bool,
    output_path: &str,
) -> Result<Repository, Error> {
    // try and get the default configs which will
    // include the Credential Helper
    let config = Config::open_default()?;

    let mut fetch_options = git2::FetchOptions::new();

    fetch_options.remote_callbacks(clone_callback(config)?);

    // TODO factor this out to a new function
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.bare(clone_bare);

    builder.clone(url, Path::new(output_path))
}

pub fn config_repo_mirror(repo_path: &str) -> Result<(), Error> {
    let repo = Repository::open(repo_path)?;

    let config = repo.config();
    match config {
        Ok(mut c) => c.set_bool("remote.origin.mirror", true),
        Err(e) => Err(e),
    }
}

pub fn set_repo_push_url(
    url: &str,
    name: &str,
    repo_path: &str,
) -> Result<(), Error> {
    let repo = Repository::open(repo_path)?;

    repo.remote_set_pushurl(name, Some(url))
}

pub fn add_repo_remote<'repo>(
    repo: &'repo Repository,
    name: &str,
    remote_url: &str,
) -> Result<Remote<'repo>, Error> {
    repo.remote(name, remote_url)
}

pub fn set_repo_remote_url(
    repo: &Repository,
    name: &str,
    url: &str,
) -> Result<(), Error> {
    repo.remote_set_url(name, url)
}

pub fn push_repo(repo: Repository) -> Result<(), Error> {
    let mut r = repo.find_remote("origin")?;
    let config = Config::open_default()?;
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(clone_callback(config)?);

    //let branch_names = get_branch_names(&repo)?;
    // let ref_specs = get_ref_spec(branch_names);
    // println!("{:?}", ref_specs);

    r.push(
        &[format!("refs/heads/{}", get_head(&repo)?)],
        Some(&mut push_options),
    )?;
    //r.push(&["refs/heads/master"],Some(&mut push_options))?;

    Ok(())
}

#[allow(unused)]
fn read_repo_migrations_params(param_path: &Path) -> Result<(), Error> {
    Ok(())
}
