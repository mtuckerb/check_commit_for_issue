# Check commit for Issue

## Overview
This little app is meant to be used in a git hook. 
It will verify that your commit message begins with the Issue number, 
and that that issue number is a valid Jira issue in the current sprint.

If you have redis running on `localhost`, it will cache the API responses, making things a lot faster.

## Installation
1. Build your binary; 
    ```
    cargo build --release
    ```
2. Copy that into your local git repo; 
    ```
    cp ./target/release/check_commit_for_issue <path_to_repo>/.git/hooks
    ```
4. Copy the sample hook into your local git repo;
   ```
    cp hooks/commit_msg <path_to_repo>/.git/hooks
   ```
   or copy/paste the contents
5. Run the binary without any arguments to configure it;
    ```
    <path_to_repo>/.git/hooks/check_commit_for_issue
    ``` 

That's all. 