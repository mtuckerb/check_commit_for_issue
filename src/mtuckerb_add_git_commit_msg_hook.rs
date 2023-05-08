use std::path::Path;
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
      .read(true)
      .append(true)
      .mode(0o775)
      .open(".git/hooks/commit-msg")
      .unwrap();

      // return Ok if the file contains the string /usr/local/bin/check_commit_for_issue
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      if contents.contains("/usr/local/bin/check_commit_for_issue") {
        println!("Hook is already installed. Skipping…");
        return Ok(contents);
      }

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

// write tests for this function
#[cfg(test)]
mod tests {
  use super::*;

  
  #[test]
  fn test_add_hook() {
    let _ = std::fs::remove_file(".git/hooks/commit-msg");
    assert_eq!(add_hook().unwrap(), "Hook added!".to_string());
    assert!(add_hook().unwrap().contains(r#"
    test "$(/usr/local/bin/check_commit_for_issue "$1")" || {
      echo >&2 Invalid Commit Message
      exit 1
    }
  "#));
  }
}