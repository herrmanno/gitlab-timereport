use std::collections::HashSet;

use thiserror::Error;

use crate::{
    model::{Issue, MergeRequest, Milestone, Project, TimeLog, User},
    FetchError, FetchResult,
};

mod group;
mod issues;
mod merge_requests;
// mod projects;
mod users;

trait Fetch<T, I> {
    fn fetch(&self, client: &reqwest::blocking::Client, id: I) -> anyhow::Result<T>;
}

pub(crate) struct Id(u32);

#[derive(Debug, Error)]
pub(crate) enum IdError {
    #[error("Cannot parse id from string '{0}'")]
    ParseError(String),
}

impl TryFrom<&str> for Id {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let part = value
            .split('/')
            .last()
            .ok_or_else(|| IdError::ParseError(value.to_string()))?;
        let id = part
            .parse()
            .map_err(|_| IdError::ParseError(value.to_string()))?;
        Ok(Id(id))
    }
}

pub(crate) struct Fetcher {
    base_uri: String,
    client: reqwest::blocking::Client,
}

impl Fetcher {
    pub fn new(base_uri: String, personal_access_token: String) -> anyhow::Result<Self> {
        let headers = {
            let mut h = reqwest::header::HeaderMap::new();
            h.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", personal_access_token).parse()?,
            );
            h
        };
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| FetchError::FetchError("Cannot construct HTTP client".to_string()))?;

        Ok(Fetcher { base_uri, client })
    }

    pub fn go(&self, group_name: String) -> anyhow::Result<FetchResult> {
        let group_name = group_name.replace(' ', "-");

        let (projects, milestones): (Vec<Project>, Vec<Milestone>) =
            self.fetch(&self.client, group_name.clone())?;
        let mut issues: Vec<Issue> = vec![];
        let mut time_logs: Vec<TimeLog> = vec![];
        let mut merge_requests: Vec<MergeRequest> = vec![];

        for project in projects.iter() {
            let full_project_path = format!("{}/{}", group_name, project.name.replace(' ', "-"));

            let (new_issues, new_time_logs): (Vec<Issue>, Vec<TimeLog>) =
                self.fetch(&self.client, full_project_path.clone())?;
            issues.extend(new_issues);
            time_logs.extend(new_time_logs);

            let (new_merge_requests, new_time_logs): (Vec<MergeRequest>, Vec<TimeLog>) =
                self.fetch(&self.client, full_project_path)?;
            merge_requests.extend(new_merge_requests);
            time_logs.extend(new_time_logs);
        }

        let user_ids = time_logs
            .iter()
            .map(|tl| tl.user_id)
            .collect::<HashSet<u32>>();

        let users: Vec<User> = self.fetch(&self.client, user_ids)?;

        // let projects = self.fetch(&self.client, vec![46].as_slice())?;

        Ok(FetchResult {
            projects,
            milestones,
            issues,
            merge_requests,
            time_logs,
            users,
        })
    }
}
