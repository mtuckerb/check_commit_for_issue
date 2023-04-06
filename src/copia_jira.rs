use crate::copia_redis;
use copia_redis::set_redis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    id: String,
    key: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Obj {
    issues: Vec<Issue>,
}

pub async fn lookup_issue(
    message_id: &str,
    auth_token: &str,
    board_id: &str,
) -> Result<bool, bool> {
    let sprint = reqwest::Client::new()
        .get(format!(
            "https://gocopia.atlassian.net/rest/agile/1.0/board/{}/sprint?state=active",
            &board_id.to_string()
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", &auth_token))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await;
    let sprint_id = &sprint.unwrap()["values"][0]["id"].to_string();

    let issues = reqwest::Client::new()
        .get(format!(
            "https://gocopia.atlassian.net/rest/agile/1.0/board/71/sprint/{}/issue",
            sprint_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", auth_token))
        .send()
        .await
        .expect("Error fetching Issues")
        .json::<Obj>()
        .await;

    let mut result: bool = false;

    for i in &issues.unwrap().issues {
        set_redis(&i.key);
        if i.key.eq(message_id) {
            println!("{:#?} is in the current Sprint. Nice!", &i.key);
            result = true;
            break;
        };
    }
    return match result {
        true => Ok(result),
        false => Err(result),
    };
}
