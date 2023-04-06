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

#[tokio::main]
async fn main() -> Result<(), ()> {
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
                return Err(());
            }
        },
        None => {
            println!("Your Issue does not appear to start with an Issue");
            return Err(());
        }
    };

    let issue_found: Result<bool, bool>;

    match check_redis(&message_id) {
        Ok(_) => {
            return Ok(());
        }
        Err(_) => issue_found = lookup_issue(&message_id, &auth_token, &config).await,
    }

    return match issue_found {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    };
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
