use reqwest::header::HeaderMap;

use crate::github_event::*;
use std::{env, process::Command};

pub fn github_event_repo_url() -> String {
  let repo = parseEnv("GITHUB_REPOSITORY");
  let api_url = parseEnv("GITHUB_API_URL");

  format!("{}/repos/{}", api_url, repo)
}

pub fn parseEnv(env: &str) -> String {
  env::var_os("GITHUB_TOKEN")
    .unwrap()
    .into_string()
    .expect("Invalid environment variable")
}

pub fn git(args: Vec<&str>) {
  Command::new("git")
    .args(args)
    .output()
    .expect("git command failed");
}

pub fn git_setup(token: String) {
  let repo = parseEnv("GIT_REPO");
  let actor = parseEnv("GITHUB_ACTOR");

  let url = format!("https://{}:{}@github.com/{}.git", actor, token, repo);

  git(["remote", "set-url", "--push", "origin", url.as_str()].to_vec());

  git(["config", "user.name", "github action"].to_vec());
  git(["config", "user.email", "action@github.com"].to_vec());
}

pub fn get_github_api_headers(token: String) -> HeaderMap {
  let mut headers: HeaderMap = HeaderMap::new();

  headers.append("Authorization", r#"Bearer {token}"#.parse().unwrap());
  headers.append("content-type", "application/json".parse().unwrap());
  headers.append("accept", "application/vnd.github.v3+json".parse().unwrap());
  headers
}

#[tokio::main]
pub async fn github_get_commits_in_pr(prNumber: i64, token: String) -> Vec<String> {
  let headers = get_github_api_headers(token);
  let repoUrl = github_event_repo_url();
  let client = reqwest::Client::new();
  let mut commits = Vec::new();

  let url = format!("{}/pulls/{}/commits", repoUrl, prNumber);

  let response = client
    .get(url)
    .headers(headers)
    .send()
    .await
    .expect("Failed to get commits")
    .json::<Vec<GithubGetCommitResponseItem>>()
    .await
    .expect("Failed to parse json");

  for commit in response {
    commits.push(commit.sha);
  }
  commits
}
