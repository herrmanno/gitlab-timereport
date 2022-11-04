use graphql_client::{GraphQLQuery, Response};

use super::{Fetch, Fetcher};
use crate::{
    graphql::Id,
    model::{MergeRequest, TimeLog},
    FetchError,
};

type Time = String;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./gitlab_schema.json",
    query_path = "./merge_requests_query.graphql",
    response_derives = "Debug",
    "Clone",
    "Default",
    skip_serializing_none
)]
pub(crate) struct MergeRequestsQuery;

impl Fetch<(Vec<MergeRequest>, Vec<TimeLog>), String> for Fetcher {
    fn fetch(
        &self,
        client: &reqwest::blocking::Client,
        full_path: String,
    ) -> anyhow::Result<(Vec<MergeRequest>, Vec<TimeLog>)> {
        use merge_requests_query as q;

        let mut merge_requests = vec![];
        let mut time_logs = vec![];

        let mut cursor: Option<String> = None;
        loop {
            let query = MergeRequestsQuery::build_query(q::Variables {
                full_path: full_path.clone(),
                cursor: cursor.clone(),
            });
            let resp = client.post(&self.base_uri).json(&query).send()?;
            let response_body: Response<q::ResponseData> = resp.json()?;
            let data: q::ResponseData = response_body.data.ok_or_else(|| {
                FetchError::FetchError("No data on group query response".to_string())
            })?;
            let project = data
                .project
                .ok_or_else(|| FetchError::from("No project data in response"))?;
            let Id(project_id) = Id::try_from(project.id.as_str())?;

            let merge_requests_obj = project.merge_requests.ok_or_else(|| {
                FetchError::from(
                    "No merge_requests data on mergerequests query response's project object",
                )
            })?;

            let has_next_page = merge_requests_obj.page_info.has_next_page;
            let end_cursor = merge_requests_obj.page_info.end_cursor.clone();

            let merge_request_vec = merge_requests_obj.nodes.ok_or_else(|| {
                FetchError::from(
                    "No nodes data on mergerequests query response's project.merge_requests obejct",
                )
            })?;

            for merge_request in merge_request_vec.iter().flatten() {
                let Id(id) = Id::try_from(merge_request.id.as_str())?;
                let iid = merge_request.iid.parse()?;
                let milestone_id = match merge_request.milestone.as_ref() {
                    Some(ms) => {
                        let Id(id) = Id::try_from(ms.id.as_str())?;
                        Some(id)
                    }
                    None => None,
                };
                merge_requests.push({
                    MergeRequest {
                        id,
                        iid,
                        project_id,
                        name: merge_request.title.clone(),
                        milestone_id,
                    }
                });

                let merge_request_id = merge_requests.last().map(|mr| mr.id);

                for time_log in merge_request
                    .timelogs
                    .nodes
                    .as_ref()
                    .ok_or_else(|| {
                        FetchError::FetchError("No 'nodes' in timeLog found".to_string())
                    })?
                    .iter()
                    .flatten()
                {
                    time_logs.push({
                        let Id(user_id) = Id::try_from(time_log.user.id.as_str())?;
                        let date = time_log
                            .spent_at
                            .as_ref()
                            .ok_or_else(|| {
                                FetchError::FetchError("No date at time log".to_string())
                            })?
                            .clone();
                        TimeLog {
                            time: time_log.time_spent as i32 / 60,
                            user_id,
                            date,
                            issue_id: None,
                            merge_request_id,
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

        Ok((merge_requests, time_logs))
    }
}
