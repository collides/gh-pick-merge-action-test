mod github_event;
mod helpers;

use crate::github_event::GithubEventAction;
use chrono::prelude::*;
use helpers::github_get_commits_in_pr;
use std::{env, fs};

use helpers::*;

#[tokio::main]
async fn main() {
  let token = parse_env("GITHUB_TOKEN");

  git_setup(token.clone());

  let github_event_path = env::var_os("GITHUB_EVENT_PATH").unwrap();
  let github_event_string =
    fs::read_to_string(github_event_path).expect("read to string is failed");

  let github_event: GithubEventAction =
    serde_json::from_str(&github_event_string).expect("convert to github event is failed");

  let base_branch = github_event.pull_request.base._ref;

  let pr_number = github_event.number;

  let new_branch_name =
    create_new_branch_by_commits(base_branch.clone(), pr_number, token.clone()).await;

  let pr_title = format!("chore: backport {}", pr_number);

  let body = "auto pick merge".to_string();

  github_open_pull_request(token, new_branch_name, base_branch, pr_title, body).await;
}

async fn create_new_branch_by_commits(to_branch: String, pr_number: i64, token: String) -> String {
  let commits = github_get_commits_in_pr(pr_number, token).await;

  let utc: DateTime<Utc> = Utc::now();

  let new_branch_name = format!("bot/auto-pick-{}-{:?}", to_branch, utc);
  let origin_to_branch_name = format!("origin/{}", to_branch);

  git(
    [
      "switch",
      "-c",
      new_branch_name.as_str(),
      origin_to_branch_name.as_str(),
    ]
    .to_vec(),
  );

  println!("New branch name:{}", new_branch_name);

  for commit_hash in commits {
    println!("commit: {:?}", commit_hash);
    git(["cherry-pick", commit_hash.as_str()].to_vec());
  }

  new_branch_name.to_string()
}

#[test]
fn test_push() {
  let push = git(["push", "-u", "origin", "zyh/test1"].to_vec());

  println!("{:?}", String::from_utf8(push.stderr).unwrap());
}

#[test]
fn test_date() {
  let now: DateTime<Utc> = Utc::now();

  println!("UTC now is: {}", now);
  println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
  println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
  println!(
    "UTC now in a custom format is: {}",
    now.format("%a %b %e %T %Y")
  );
}
