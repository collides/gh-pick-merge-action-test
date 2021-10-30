use reqwest::header::HeaderMap;

use crate::github_event::*;
use std::{env, process::{Command, Output}};

pub fn github_event_repo_url() -> String {
  let repo = parse_env("GITHUB_REPOSITORY");
  let api_url = parse_env("GITHUB_API_URL");

  format!("{}/repos/{}", api_url, repo)
}

pub fn parse_env(key: &str) -> String {
  env::var_os(key)
    .expect("Environment variable is undefined")
    .into_string()
    .expect("Environment into string is failed")
}

pub fn git(args: Vec<&str>) -> Output {
  Command::new("git")
    .args(args)
    .output()
    .expect("git command failed")
}

pub fn git_setup(github_token: String) {
  let repo = parse_env("GITHUB_REPOSITORY");
  let actor = parse_env("GITHUB_ACTOR");
  // https://github.com/collides/gh-pick-merge-action.git

  println!("{:?}", repo);

  let url = format!("https://{}:{}@github.com/{}.git", actor, github_token, repo);

  git(["remote", "set-url", "--push", "origin", url.as_str()].to_vec());

  git(["config", "user.email", "action@github.com"].to_vec());
  git(["config", "user.name", "github action"].to_vec());

  let remote = git(["remote", "-v"].to_vec()).stdout;

  println!("{:?}", String::from_utf8(remote).unwrap());

}

pub fn get_github_api_headers(token: String) -> HeaderMap {
  let mut headers: HeaderMap = HeaderMap::new();

  let authorization = format!("token {}", token);

  headers.append("User-Agent", "gh-pick-merge-action".parse().unwrap());
  headers.append("Authorization", authorization.parse().unwrap());
  headers.append("content-type", "application/json".parse().unwrap());
  headers.append("accept", "application/vnd.github.v3+json".parse().unwrap());
  headers
}

pub async fn github_open_pull_request(
  token: String,
  head: String,
  base: String,
  title: String,
  body: String,
) {
  let headers = get_github_api_headers(token);
  let client = reqwest::Client::new();
  let repo_url = github_event_repo_url();

  let body = format!(
    "`{{`head:{},base:{},title:{},body:{}`}}`",
    head, base, title, body
  );

  let url = format!("{}/pulls", repo_url);

  client
    .post(url)
    .body(body)
    .headers(headers)
    .send()
    .await
    .expect("Failed to create pull request");

  println!("Create pull request success!");
}

pub async fn github_get_commits_in_pr(pr_number: i64, token: String) -> Vec<String> {
  let headers = get_github_api_headers(token);
  let repo_url = github_event_repo_url();
  let client = reqwest::Client::new();
  let mut commits = Vec::new();

  let url = format!("{}/pulls/{}/commits", repo_url, pr_number);

  let response = client
    .get(url)
    .headers(headers)
    .send()
    .await
    .expect("Failed to get commits")
    .json::<Vec<GithubGetCommitResponseItem>>()
    .await
    .expect("Failed into json by commit");

  for commit in response {
    commits.push(commit.sha);
  }
  commits
}
