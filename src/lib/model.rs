use serde::Deserialize;

pub(crate) trait Unique {
    type Key: PartialEq + Eq + PartialOrd + Ord;

    fn get_key(&self) -> Self::Key;
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Group {
    pub(crate) id: u32,
    pub(crate) projects: Vec<Project>,
}

impl Unique for Group {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Project {
    pub(crate) id: u32,
    pub(crate) name: String,
}

impl Unique for Project {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Milestone {
    pub(crate) id: u32,
    pub(crate) name: String,
}

impl Unique for Milestone {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Issue {
    pub(crate) id: u32,
    pub(crate) iid: u32,
    pub(crate) name: String,
    pub(crate) project_id: u32,
    pub(crate) milestone_id: Option<u32>,
}

impl Unique for Issue {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct MergeRequest {
    pub(crate) id: u32,
    pub(crate) iid: u32,
    pub(crate) name: String,
    pub(crate) project_id: u32,
    pub(crate) milestone_id: Option<u32>,
}

impl Unique for MergeRequest {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct TimeLog {
    pub(crate) time: i32,
    pub(crate) date: String,
    pub(crate) user_id: u32,
    pub(crate) issue_id: Option<u32>,
    pub(crate) merge_request_id: Option<u32>,
}

impl Unique for TimeLog {
    type Key = (u32, String);

    fn get_key(&self) -> Self::Key {
        (self.user_id, self.date.clone())
    }
}

#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) username: String,
}

impl Unique for User {
    type Key = u32;

    fn get_key(&self) -> Self::Key {
        self.id
    }
}
