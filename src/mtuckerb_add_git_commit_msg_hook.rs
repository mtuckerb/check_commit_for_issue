use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;

pub fn add_hook() -> Result<String, String> {
  let b: bool = Path::new(".git").is_dir();
  match b {
    true => "",
    false => return Err(".git not found, are you in the project root?".to_string())
  };
  
  
  let mut file = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .mode(0o775)
      .open(".git/hooks/commit-msg")
      .unwrap();



  if let Err(e) = writeln!(file, "{}", r#"
    test "$(/usr/local/bin/check_commit_for_issue "$1")" || {
      echo >&2 Invalid Commit Message
      exit 1
    }
  "#) {
      return Err(format!("Couldn't write to file: {}", e));
  }
  Ok("Hook added!".to_string())

}
