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

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Git Rusty",
    about = "Example Git operations in Rust.  License: https://www.gnu.org/licenses/agpl-3.0.en.html"
)]
pub struct CliOptions {
    #[structopt(long = "repo-url")]
    pub repo_url: String,
    #[structopt(long = "output-path")]
    pub output_path: String,
    #[structopt(long = "git-hub-username")]
    pub username: String,
    #[structopt(long = "new-repo-name")]
    pub new_repo_name: String,
    #[structopt(
        long = "github-pat",
        env = "GIT_HUB_PAT",
        hide_env_values = true
    )]
    pub github_pat: String,
}
