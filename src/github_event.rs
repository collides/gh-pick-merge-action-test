use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubEventActionBase {
  pub _ref: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubEventActionPullRequest {
  pub number: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubEventAction {
  action: String,
  pub base: GithubEventActionBase,
  pub pull_request: GithubEventActionPullRequest
}




// ------ Response ------

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubGetCommitResponseItem {
  pub sha: String,
}