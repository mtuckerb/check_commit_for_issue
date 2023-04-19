use regex::Regex;
use std::env;
use std::fs;
extern crate base64;
mod get_config;
mod mtuckerb_jira;
mod mtuckerb_redis;
use base64::{engine::general_purpose, Engine as _};
use get_config::get_config;
use mtuckerb_jira::lookup_issue;
use mtuckerb_redis::check_redis;
use colored::Colorize;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    use std::io::{self, Write};
    match real_main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            let stderr = io::stderr();
            let _ = writeln!(&mut &stderr, "Error: {e}");
            ExitCode::FAILURE
        }
    }
}


async fn real_main() -> Result<(), String> {
    let config: get_config::MtuckerbConfig = get_config().await;
    let file_name = get_filename();
    let auth_token = general_purpose::STANDARD_NO_PAD
        .encode(format!("{}:{}", &config.jira_email, &config.jira_password));
    let contents = fs::read_to_string(&file_name)
        .expect("Should have been able to read the file have you made a commit yet?");
    let re = Regex::new(r"^(?P<issue_no>\w+-\d+) ").unwrap();

    let message_id = match re.captures(&contents) {
        Some(m) => match m.name("issue_no") {
            Some(mes) => mes.as_str(),
            None => {
                println!("Your Issue does not appear to start with an Issue");
                return Err("Your Issue does not appear to start with an Issue".to_owned());
            }
        },
        None => {
            println!("Your Issue does not appear to start with an Issue");
            return Err("Your Issue does not appear to start with an Issue".to_owned());
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
        Err(e) => Err(format!("{}", e.red().bold() )),
    }
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let file_name: String = if args.len() > 0 {
        args[1].clone()
    } else {
        "./git/COMMIT_EDITMSG".to_string()
    };
    file_name
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
