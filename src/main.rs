mod helpers;

use std::env;

use helpers::*;

fn main() {
  let token = env::var_os("GITHUB_TOKEN").unwrap().into_string().unwrap();

  
    git_setup(token);
    println!("Hello, world!");
}
