use anyhow::Context;
use graphql_client::{GraphQLQuery, Response};

use super::{Fetch, Fetcher};
use crate::{
    graphql::Id,
    model::{Milestone, Project},
    FetchError,
};

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./gitlab_schema.json",
    query_path = "./group_query.graphql",
    response_derives = "Debug",
    "Clone",
    "Default",
    skip_serializing_none
)]
pub(crate) struct GroupQuery;

impl Fetch<(Vec<Project>, Vec<Milestone>), String> for Fetcher {
    fn fetch(
        &self,
        client: &reqwest::blocking::Client,
        full_path: String,
    ) -> anyhow::Result<(Vec<Project>, Vec<Milestone>)> {
        use group_query as q;

        let query = GroupQuery::build_query(q::Variables { full_path });
        let resp = client
            .post(&self.base_uri)
            .json(&query)
            .send()
            .context("Cannot make API request")?;
        let response_body: Response<q::ResponseData> =
            resp.json().context("API respone is no valid JSON")?;
        let data: q::ResponseData = response_body
            .data
            .ok_or_else(|| FetchError::FetchError("Group response is empty".to_string()))?;
        let group = data
            .group
            .ok_or_else(|| FetchError::FetchError("Group response is empty".to_string()))?;
        let projects = group.projects.edges.ok_or_else(|| {
            FetchError::FetchError("Group response contains no projects".to_string())
        })?;

        let group_milestones: Vec<Milestone> = match group.milestones {
            Some(ms) => ms
                .edges
                .unwrap_or_default()
                .iter()
                .flatten()
                .flat_map(|edge| edge.node.as_ref())
                .filter_map(|node| match Id::try_from(node.id.as_str()) {
                    Ok(Id(id)) => Some(Milestone {
                        id,
                        name: node.title.clone(),
                    }),
                    _ => None,
                })
                .collect(),
            None => Default::default(),
        };

        let project_milestones = projects
            .iter()
            .flatten()
            .flat_map(|edge| &edge.node)
            .flat_map(|node| {
                let project_milestones = node.milestones.as_ref()?;
                let nodes = project_milestones.nodes.as_ref()?;
                let vec = nodes.iter().flatten().filter_map(|node| {
                    match Id::try_from(node.id.as_str()) {
                        Ok(Id(id)) => Some(Milestone {
                            id,
                            name: node.title.clone(),
                        }),
                        _ => None,
                    }
                });
                Some(vec)
            })
            .flatten();

        let projects = projects
            .iter()
            .flatten()
            .flat_map(|edge| &edge.node)
            .filter_map(|node| match Id::try_from(node.id.as_str()) {
                Ok(Id(id)) => Some(Project {
                    id,
                    name: node.name.clone(),
                }),
                _ => None,
            })
            .collect();


        let milestones = group_milestones
            .into_iter()
            .chain(project_milestones.into_iter())
            .collect();

        Ok((projects, milestones))
    }
}
