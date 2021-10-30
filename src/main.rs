mod github_event;
mod helpers;

// use helpers::github_get_commits_in_pr;

use crate::github_event::GithubEventAction;
use std::{env, fs};

// use helpers::*;

// fn backport_commit(baseBranch: String, prNumber: i64, token: String) {
//   let commits = github_get_commits_in_pr(prNumber, token);
// }

fn main() {
  // let token = parseEnv("GITHUB_TOKEN");

  // git_setup(token);

  let github_event_path = env::var_os("GITHUB_EVENT_PATH").unwrap();
  let github_event = fs::read_to_string(github_event_path).expect("read to string is failed");

  let res: GithubEventAction =
    serde_json::from_str(&github_event).expect("convert github event is failed");

  // let base_branch = res.base._ref;

  // let pr_number = res.pull_request.number;

  // backport_commit(base_branch, pr_number, token);

  println!("Hello, world!");
  println!("{:?}", res);
}
