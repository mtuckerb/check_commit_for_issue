use crate::get_config;
use crate::mtuckerb_redis;
use mtuckerb_redis::set_redis;
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
    config: &get_config::MtuckerbConfig,
) -> Result<bool, String> {
    let sprint_id = match reqwest::Client::new()
    .get(format!(
        "https://{}.atlassian.net/rest/agile/1.0/board/{}/sprint?state=active",
        config.subdomain, config.board_id
    ))
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Basic {}", &auth_token))
    .send()
    .await {
        Ok(resp) => { 
            match resp.json::<serde_json::Value>().await {
                Ok(val) => val["values"][0]["id"].to_string(),
                Err(_) => "Couldn't find sprint_id".to_string(),
            }
        },
        Err(e) => panic!("Could not connect to Jira: {}", e),
    };


    let issues = match reqwest::Client::new()
        .get(format!(
            "https://{}.atlassian.net/rest/agile/1.0/board/{}/sprint/{}/issue?maxResults=1000",
            config.subdomain, config.board_id, sprint_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", auth_token))
        .send()
        .await {
            Ok(resp) => {
                match resp.json::<Obj>().await {
                    Ok(data) => data,
                    Err(e) => panic!("Error parsing response body: {}", e),
                }
            }
            Err(e) => panic!("Error fetching Issues: {}", e),
        };
        
    let mut result: bool = false;

    for i in &issues.issues {
        let _ = set_redis(&i.key);
        if i.key.eq(message_id) {
            println!("✅ {:#?} found in Jira!", &i.key);
            result = true;
            break;
        };
    }
    return match result {
        true => Ok(result),
        false => Err(format!("🛑 Issue not found in Jira!")),
    };
}
