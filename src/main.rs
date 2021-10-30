mod github_event;
mod helpers;

use helpers::github_get_commits_in_pr;

use crate::github_event::GithubEventAction;
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

  let new_branch_name = create_new_branch_by_commits(pr_number, token).await;

  create_new_pull_request(base_branch, new_branch_name, pr_number, token).await;

  println!("Hello, world!");
}

async fn create_new_branch_by_commits(pr_number: i64, token: String) -> String {
  let commits = github_get_commits_in_pr(pr_number, token).await;

  let new_branch_name = "zyh/test-1";
  let origin_new_branch_name = format!("origin/{}", new_branch_name).as_str();

  git(["switch", "-c", new_branch_name, origin_new_branch_name].to_vec());

  println!("new branch name:{}", new_branch_name);

  for commit_hash in commits {
    git(["cherry-pick", commit_hash.as_str()].to_vec())
  }

  git(["push", "-u", "origin", new_branch_name].to_vec());

  new_branch_name.to_string()
}

async fn create_new_pull_request(
  baseBranch: String,
  newBranch: String,
  pr_number: i64,
  token: String,
) {
  let pr_title = format!("chore: backport {}", pr_number);
  github_open_pull_request(token, newBranch, baseBranch, pr_title, "test1".to_string());
}
