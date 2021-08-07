use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Git Rusty",
            about =
            "Example Git operations in Rust.  License: https://www.gnu.org/licenses/agpl-3.0.en.html")]
pub struct CliOptions {
    #[structopt(long = "repo-url")]
    pub repo_url: String,
    #[structopt(long = "output-path")]
    pub output_path: String,
    #[structopt(long = "git-hub-username")]
    pub username: String,
    #[structopt(long = "new-repo-name")]
    pub new_repo_name: String,
    #[structopt(long = "github-pat", env = "GIT_HUB_PAT", hide_env_values = true)]
    pub github_pat: String
    
}