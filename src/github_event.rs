use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GithubEventActionBase {
  _ref: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubEventAction {
  action: String,
  base: GithubEventActionBase,
}
