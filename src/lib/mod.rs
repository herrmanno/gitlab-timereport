mod db;
mod graphql;
mod model;

use model::*;
use thiserror::Error;

#[derive(Debug, Default)]
pub(crate) struct FetchResult {
    pub(crate) projects: Vec<Project>,
    pub(crate) milestones: Vec<Milestone>,
    pub(crate) issues: Vec<Issue>,
    pub(crate) merge_requests: Vec<MergeRequest>,
    pub(crate) time_logs: Vec<TimeLog>,
    pub(crate) users: Vec<User>,
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Error while fetching API: {0}")]
    FetchError(String),
}

impl From<&str> for FetchError {
    fn from(s: &str) -> Self {
        Self::FetchError(s.to_string())
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Error while executing DB statment: {0}")]
    DbError(String),
}

pub fn go(
    base_uri: String,
    personal_access_token: String,
    group_name: String,
    db_file_path: String,
) -> anyhow::Result<()> {
    println!("URI: {}", base_uri);
    println!("Token: {}", personal_access_token);

    let fetcher = graphql::Fetcher::new(base_uri, personal_access_token)?;
    let FetchResult {
        projects,
        milestones,
        issues,
        merge_requests,
        time_logs,
        users,
    } = fetcher.go(group_name)?;

    db::save_to_db(
        db_file_path,
        projects,
        milestones,
        issues,
        merge_requests,
        time_logs,
        users,
    )?;

    Ok(())
}
