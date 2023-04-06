use confy;
use futures_util::StreamExt;
use serde_derive::{Deserialize, Serialize};
use tokio::io;
use tokio_util::codec::{FramedRead, LinesCodec};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CopiaConfig {
    pub jira_email: String,
    pub jira_password: String,
    pub board_id: String,
    pub subdomain: String,
}

pub async fn get_config() -> CopiaConfig {
    let config: CopiaConfig = match confy::load("copia", "check_commit_for_issue") {
        Ok(cfg) => cfg,
        Err(_) => {
            let config = CopiaConfig::default();
            return set_config(config).await;
        }
    };

    if !(config.jira_email == "")
        || !(config.jira_password == "")
        || !(config.board_id == "")
        || !(config.subdomain == "")
    {
        return config;
    }

    set_config(config).await
}

async fn set_config(mut config: CopiaConfig) -> CopiaConfig {
    let file = confy::get_configuration_file_path("copia", "check_commit_for_issue").unwrap();
    println!("Configuration file path is: {:#?}", file);

    let stdin = io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    if config.jira_email == "" {
        println!("\nPlease enter your Jira email and hit <cr>:");
        config.jira_email = reader.next().await.transpose().unwrap().unwrap();
    }
    if config.jira_password == "" {
        println!("\nNow please enter the API token that you created at\n https://id.atlassian.com/manage-profile/security/api-tokens\n and hit <cr>:");
        config.jira_password = reader.next().await.transpose().unwrap().unwrap();
    }
    if config.board_id == "" {
        println!("\nOkay! please enter the board id that you want to check\n");
        config.board_id = reader.next().await.transpose().unwrap().unwrap();
    }
    if config.subdomain == "" {
        println!("\nLastly, please enter the subdomain for your Atlassian cloud");
        config.subdomain = reader.next().await.transpose().unwrap().unwrap();
    }
    confy::store("copia", "check_commit_for_issue", &config)
        .expect("Failed to store configuration");
    return config;
}
