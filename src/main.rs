use regex::Regex;
use std::env;
use std::fs;
extern crate base64;
mod get_config;
mod mtuckerb_jira;
mod mtuckerb_redis;
mod mtuckerb_add_git_commit_msg_hook;

use base64::{engine::general_purpose, Engine as _};
use colored::Colorize;
use get_config::{get_config, set_config};
use mtuckerb_jira::lookup_issue;
use mtuckerb_redis::check_redis;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    use std::io::{self, Write};
    match real_main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            let stderr = io::stderr();
            let _ = writeln!(&mut &stderr, "{e}");
            ExitCode::FAILURE
        }
    }
}

async fn real_main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--config".to_string()) || args.len() < 1 {
        println!("launching config");
        set_config().await;
    }

    let config: get_config::MtuckerbConfig = get_config().await;
    
    let file_name = match get_filename() {
        Ok(file_name) => file_name,
        Err(..) => {
            return Err(format!(
                "{}",
                "Please provide a valid filename.".red().bold()
            ));
        }
    };
   

    let auth_token = general_purpose::STANDARD_NO_PAD
        .encode(format!("{}:{}", &config.jira_email, &config.jira_password));
    let contents = match fs::read_to_string(&file_name) {
        Ok(content) => content,
        Err(..) => {
            return Err(format!("{}", "Please provide a valid filename. e.g.: check_commit_for_issues .git/COMMIT_EDITMSG".red().bold()));
        }
    };
    // .expect("Should have been able to read the file have you made a commit yet?");
    let re = Regex::new(r"^(?P<issue_no>\w+-\d+) ").unwrap();

    let message_id = match re.captures(&contents) {
        Some(m) => match m.name("issue_no") {
            Some(mes) => mes.as_str(),
            None => {
                return Err(format!(
                    "{}",
                    "Your commit does not appear to start with an Issue"
                        .red()
                        .bold()
                ));
            }
        },
        None => {
            return Err(format!(
                "{}",
                "Your commit does not appear to start with an Issue"
                    .red()
                    .bold()
            ));
        }
    };

    let issue_found: Result<bool, String>;

    match check_redis(&message_id) {
        Ok(msg) => {
            println!("✅ {}", msg.green().bold());
            return Ok(());
        }
        Err(e) => {
            println!("{} {}", "Reids lookup failed:".red().bold(), e.red().bold());
            issue_found = lookup_issue(&message_id, &auth_token, &config).await;
        }
    }

    match issue_found {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e.red().bold())),
    }
}

fn get_filename() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        return Ok(args[1].clone());
    } else {
        return Ok("./git/COMMIT_EDITMSG".to_string());
    };
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[automock]
//     trait Filename {
//         fn get_filename(valid: bool) -> String {
//             return "./src/test_files/valid".to_string();
//         }
//     }
//     #[tokio::test]
//     async fn it_works() {
//         let mut mock = MockFilename::new();
//         assert_eq!(Ok(()), main().await);
//     }
// }
