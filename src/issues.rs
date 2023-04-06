pub mod issues {
    #[derive(Debug, Clone)]
    pub struct Issue {
        pub id: String,
    }

    pub fn get() -> Vec<Issue> {
        let mut vec = Vec::new();
        let issue = Issue {
            id: String::from("abc123"),
        };
        vec.push(issue);
        return vec;
    }
}
