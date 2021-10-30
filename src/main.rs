mod github_event;
mod helpers;

use crate::github_event::GithubEventAction;
use std::{env, fs};

// use helpers::*;

fn main() {
  // let token = parseEnv("GITHUB_TOKEN");

  // git_setup(token);

  let github_event_path = env::var_os("GITHUB_EVENT_PATH").unwrap();
  let github_event = fs::read_to_string(github_event_path).expect("read to string is failed");

  let res: GithubEventAction =
    serde_json::from_str(&github_event).expect("convert github event is failed");

  println!("{:?}", res);

  println!("Hello, world!");
}
