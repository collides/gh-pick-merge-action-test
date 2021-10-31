mod github_event;
mod helpers;

use chrono::prelude::*;
use helpers::github_get_commits_in_pr;

use helpers::*;

#[tokio::main]
async fn main() {
  git_setup();

  let github_event = get_event_action();

  let pr_number = github_event.number;

  let new_branch_name = create_new_branch_by_commits("develop".to_string(), pr_number)
    .await
    .expect("Create new branch by commit is failed");

  let pr_title = format!("chore: auto pick {}", pr_number);

  let body = "auto pick merge".to_string();

  github_open_pull_request(new_branch_name, "develop".to_string(), pr_title, body).await;
  github_pull_request_push_comment(pr_number, "test".to_string()).await;
}

fn generate_new_branch_name(to_branch: String) -> String {
  let timestamp: i64 = Utc::now().timestamp();

  format!("bot/auto-pick-{}-{:?}", to_branch, timestamp)
}

fn format_error_pick_comment(hash: Vec<String>) -> String {
  format!("Need to handle errors: {:?}", hash)
}

async fn create_new_branch_by_commits(to_branch: String, pr_number: i64) -> Option<String> {
  let origin_to_branch_name = format!("origin/{}", to_branch);

  let new_branch_name = generate_new_branch_name(to_branch);

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

  let not_matched_hash = pick_commits(pr_number).await;

  if not_matched_hash.len() > 0 {
    let comment = format_error_pick_comment(not_matched_hash);

    github_pull_request_push_comment(pr_number, comment).await;
    return None;
  }

  git(["push", "-u", "origin", new_branch_name.as_str()].to_vec());

  Some(new_branch_name)
}

async fn pick_commits(pr_number: i64) -> Vec<String> {
  let mut is_error = false;
  let mut not_matched_hash = Vec::new();
  let commits = github_get_commits_in_pr(pr_number).await;

  for commit_hash in commits {
    if is_error == true {
      not_matched_hash.push(commit_hash);
      continue;
    }

    let output = git(["cherry-pick", commit_hash.as_str()].to_vec());

    match output {
      Some(_output) => {
        println!("Pick success Commit hash: {:?}", commit_hash);
      }
      None => {
        is_error = true;
      }
    }
  }

  not_matched_hash
}
