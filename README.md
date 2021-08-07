# Git-Rusty
Learning Example of Git and GitHub operations in Rust.
* Clone a Git repo from GitHub 
* Add a new mirror to the cloned repo
* Create an empty private repo on GitHub
* Push the cloned repo to the new repo
* Get a repo from GitHub - returns the status code 

## Caveats
This code is working as learning examples, use at own risk.

**Several Assumptions are Made**
* The directory where the input repo is cloned is empty - will panic otherwise
* A Git Credential Helper is set with cached credentials for your Git remote
* This code can find the default Git config to know about your configured Credential Helper
* A GitHub Personal Access Token has been configured with repo create permissions, and is provided to this code
* See command line help for details

## TODOs
* Clean up/fix unwraps in a couple of functions, proper error handling
* DONE - Get the newly created Repo on GitHub isn't working
* DONE - Check path passed in for CLI option output-path and validate empty
* DONE - For Git push, accommodate non default refspecs

## Usage 

Note: A GitHub Personal Access Token can be set as an environment variable (GIT_HUB_PAT).

```
git-rusty 0.1.0

USAGE:
    git-rusty --github-pat <github-pat> --new-repo-name <new-repo-name> --output-path <output-path> --repo-url <repo-url> --git-hub-username <username>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --github-pat <github-pat>           [env: GIT_HUB_PAT]
        --new-repo-name <new-repo-name>    
        --output-path <output-path>        
        --repo-url <repo-url>              
        --git-hub-username <username>   

```
