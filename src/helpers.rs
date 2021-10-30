use std::{env, process::Command};

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

pub fn get_base_branch() {}
