use graphql_client::{GraphQLQuery, Response};

use super::{Fetch, Fetcher};
use crate::{
    graphql::Id,
    model::{Issue, TimeLog},
    FetchError,
};

type Time = String;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./gitlab_schema.json",
    query_path = "./issues_query.graphql",
    response_derives = "Debug",
    "Clone",
    "Default",
    skip_serializing_none
)]
pub(crate) struct IssuesQuery;

impl Fetch<(Vec<Issue>, Vec<TimeLog>), String> for Fetcher {
    fn fetch(
        &self,
        client: &reqwest::blocking::Client,
        full_path: String,
    ) -> anyhow::Result<(Vec<Issue>, Vec<TimeLog>)> {
        use issues_query as q;

        let mut issues = vec![];
        let mut time_logs = vec![];

        let mut cursor: Option<String> = None;
        loop {
            let query = IssuesQuery::build_query(q::Variables {
                full_path: full_path.clone(),
                cursor: cursor.clone(),
            });
            let resp = client.post(&self.base_uri).json(&query).send()?;
            let response_body: Response<q::ResponseData> = resp.json()?;
            let data: q::ResponseData = response_body
                .data
                .ok_or_else(|| FetchError::from("No data on issues query response"))?;
            let project = data
                .project
                .ok_or_else(|| FetchError::from("No project data on issues query response"))?;
            let Id(project_id) = Id::try_from(project.id.as_str())?;
            let issues_obj = project.issues.ok_or_else(|| {
                FetchError::from("No issues data on issues query response's project object")
            })?;

            let has_next_page = issues_obj.page_info.has_next_page;
            let end_cursor = issues_obj.page_info.end_cursor.clone();

            let issues_vec = issues_obj.nodes.ok_or_else(|| {
                FetchError::from("No nodes data on issues query response's project.issues obejct")
            })?;

            for issue in issues_vec.iter().flatten() {
                let Id(id) = Id::try_from(issue.id.as_str())?;
                let iid = issue.iid.parse()?;
                let milestone_id = match issue.milestone.as_ref() {
                    Some(ms) => {
                        let Id(id) = Id::try_from(ms.id.as_str())?;
                        Some(id)
                    }
                    None => None,
                };
                issues.push(Issue {
                    id,
                    iid,
                    project_id,
                    name: issue.title.clone(),
                    milestone_id,
                });

                let issue_id = issues.last().map(|i| i.id);

                for time_log in issue
                    .timelogs
                    .nodes
                    .as_ref()
                    .ok_or_else(|| FetchError::from("No 'nodes' in timeLog found"))?
                    .iter()
                    .flatten()
                {
                    let Id(user_id) = Id::try_from(time_log.user.id.as_str())?;
                    let date = time_log
                        .spent_at
                        .as_ref()
                        .ok_or_else(|| FetchError::from("No date at time log"))?
                        .clone();
                    time_logs.push({
                        TimeLog {
                            time: time_log.time_spent as i32 / 60,
                            user_id,
                            date,
                            issue_id,
                            merge_request_id: None,
                        }
                    });
                }
            }

            if has_next_page {
                cursor = end_cursor.clone();
            } else {
                break;
            };
        }

        Ok((issues, time_logs))
    }
}
