/// Currently unused, because fetching single projects is not supported yet
use graphql_client::{GraphQLQuery, Response};

use super::{Fetch, Fetcher};
use crate::{model::Project, graphql::{Id, IdError}};

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./gitlab_schema.json",
    query_path = "./projects_query.graphql",
    response_derives = "Debug",
    "Clone",
    skip_serializing_none
)]
pub(crate) struct ProjectsQuery;

impl Fetch<Vec<Project>, &[u32]> for Fetcher {
    fn fetch(
        &self,
        client: &reqwest::blocking::Client,
        ids: &[u32],
    ) -> anyhow::Result<Vec<Project>> {
        use projects_query as q;

        let project_ids = ids
            .iter()
            .map(|id| format!("gid://gitlab/Project/{}", id))
            .collect();

        let query = ProjectsQuery::build_query(q::Variables { project_ids });
        let resp = client.post(&self.base_uri).json(&query).send()?;
        let response_body: Response<q::ResponseData> = resp.json()?;
        let data: q::ResponseData = response_body.data.unwrap();
        let projects: q::ProjectsQueryProjects = data.projects.unwrap();
        let nodes = projects.nodes.unwrap();

        let result = nodes
            .into_iter()
            .flatten()
            .try_fold(vec![], |mut vec, node| {
                let Id(id) = Id::try_from(node.id.as_str())?;
                vec.push(Project {
                    id,
                    name: node.name,
                });
                Ok::<Vec<Project>, IdError>(vec)
            })?;

        Ok(result)
    }
}
