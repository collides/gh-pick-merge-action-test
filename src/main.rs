mod github_event;
mod helpers;

use crate::github_event::GithubEventAction;
use helpers::github_get_commits_in_pr;
use std::{env, fs};

use helpers::*;

#[tokio::main]
async fn main() {
  let token = parse_env("GITHUB_TOKEN");

  git_setup(token.clone());

  let github_event_path = env::var_os("GITHUB_EVENT_PATH").unwrap();
  let github_event = fs::read_to_string(github_event_path).expect("read to string is failed");

  let res: GithubEventAction =
    serde_json::from_str(&github_event).expect("convert github event is failed");

  let base_branch = res.pull_request.base._ref;

  let pr_number = res.number;

  let new_branch_name =
    create_new_branch_by_commits(base_branch.clone(), pr_number, token.clone()).await;

  let pr_title = format!("chore: backport {}", pr_number);
  github_open_pull_request(
    token,
    new_branch_name,
    base_branch,
    pr_title,
    "test1".to_string(),
  )
  .await;

  println!("Hello, world!");
}

async fn create_new_branch_by_commits(to_branch: String, pr_number: i64, token: String) -> String {
  let commits = github_get_commits_in_pr(pr_number, token).await;

  let new_branch_name = "zyh/test-2";
  let origin_to_branch_name = format!("origin/{}", to_branch);

  git(
    [
      "switch",
      "-c",
      new_branch_name,
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
