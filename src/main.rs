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

use git2::{Error, Repository};
use structopt::StructOpt;
use git_operations::{ clone_repo,
                    set_repo_remote_url,
                    config_repo_mirror,
                    push_repo };

use github_operations::{ create_repo,
                        build_repo_params,
                        get_repo};

use std::path::PathBuf;

mod cli;
mod git_operations;
mod github_operations;

// All the bits are here to clone, modify and push a
// a Git repoistory to GitHub, following GitHub's basic instructions
// for migrating a repo to GitHub.  This code makes lots of assumptions,
// see repo README for details. 
// This is all a learning exercise, some code quality work is still needed

fn check_empty_target_dir(path: PathBuf)
                        -> Option<bool> {
    let read = path.read_dir();
    match read {
        Ok(mut rd) => Some(rd.next().is_none()),
        Err(_) => Some(false)
    }
}


#[tokio::main]
async fn main() -> Result<(),Error> {
    let args = cli::CliOptions::from_args();

    match check_empty_target_dir(PathBuf::from(&args.output_path)) {
        Some(b) => {
            if b {
                if let Ok(res) = clone_repo(&args.repo_url, true, &args.output_path) {
                    println!("Target path is: {}", res.path().display()) // fix me later not returning ()
                }
            } else {
                panic!("Directory not empty, exiting...");
            }
        },
        // not certain we can reach this path...hmmm fix me
        None => panic!("Output path not empty: {}", &args.output_path)
    }

    let repo = Repository::open(&args.output_path)?;
    
    // Move this vars inside the related functions
    let new_repo_url = format!("https://github.com/{}/{}.git",&args.username,
                               &args.new_repo_name);
    set_repo_remote_url(&repo, "origin", &new_repo_url)?;
    
    config_repo_mirror(&args.output_path)?;

    // Move this vars inside the related functions
    let create_repo_url = "https://api.github.com/user/repos";

    let create_res = create_repo(&create_repo_url,
                        &args.username,
                        &args.github_pat,
                        build_repo_params(&args.new_repo_name)).await;

    let status_code = match &create_res {
        Ok(res) => res.status().as_u16(),
        Err(_) => 500 // Fix me this isn't quite right
    };
    
    println!("Create operation returned status: {}", status_code);
    if let Ok(r) = create_res {
        if r.status().is_success() {
            let get_repo_url =
                format!("https://api.github.com/repos/{}/{}",
                        &args.username,
                        &args.new_repo_name);
            
            let res = get_repo(&get_repo_url,
                               &args.username,
                               &args.github_pat).await;
            //fix me unwrap
            push_repo(repo).unwrap();
            match res {
                Ok(r) => println!("Get new repo from GitHub success? {}",
                                    r.status().is_success()),
                Err(e) => println!("Get new repo from GitHub not success: {}", e)
            }
        }
    }
    
    Ok(())
}
