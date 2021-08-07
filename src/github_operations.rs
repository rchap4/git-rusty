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

use std::collections::HashMap;
use reqwest::header::HeaderMap;

// We only build one type of repo at the moment.  
pub fn build_repo_params(name: &str) -> HashMap<&str, &str> {
    let mut params = HashMap::new();
    params.insert("private", "true");
    params.insert("auto_init", "false");
    params.insert("name", name);
    params
}

//Fixme unwrap to Result...
fn build_header() -> HeaderMap  {
    let mut headers = HeaderMap::new();

    headers
        .insert("User-Agent",
            std::format!("{}","rust-reqwest/0.11.4").parse().unwrap());
    headers
}

pub async fn create_repo(url: &str,
                username: &str,
                password: &str,
                repo_params: HashMap<&str, &str>)
                -> Result<reqwest::Response, reqwest::Error> {

    let res = reqwest::Client::new()
    .post(url)
    .headers(build_header())
    .basic_auth(username, Some(password))
    .json(&repo_params)
    .send()
    .await?;
    Ok(res)
}


pub async fn get_repo(url: &str,
                      username: &str,
                      pw: &str)
    -> Result<reqwest::Response, reqwest::Error> {

    let res = reqwest::Client::new()
    .get(url)
    .headers(build_header())
    .basic_auth(username,Some(pw))
    .send()
    .await;
    res
}
