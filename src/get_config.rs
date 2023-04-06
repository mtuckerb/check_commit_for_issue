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
}

pub async fn get_config() -> CopiaConfig {
    let mut config: CopiaConfig = confy::load("copia", "check_commit_for_issue").unwrap();
    let file = confy::get_configuration_file_path("copia", "check_commit_for_issue").unwrap();
    println!("Configuration file path is: {:#?}", file);
    if !config.jira_email.is_empty() {
        return config;
    }
    let stdin = io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    println!("\nPlease enter your Jira email and hit <cr>:");
    config.jira_email = reader.next().await.transpose().unwrap().unwrap();
    println!("\nNow please enter the API token that you created at\n https://id.atlassian.com/manage-profile/security/api-tokens\n and hit <cr>:");
    config.jira_password = reader.next().await.transpose().unwrap().unwrap();
    println!("\nLastly, please enter the board id that you want to check\nhttps://gocopia.atlassian.net/jira/projects?selectedProjectType=software");
    config.board_id = reader.next().await.transpose().unwrap().unwrap();
    confy::store("copia", "check_commit_for_issue", &config)
        .expect("Failed to store configuration");
    config
}
