use std::collections::HashSet;

use graphql_client::{GraphQLQuery, Response};

use super::{Fetch, Fetcher};
use crate::{
    graphql::{Id, IdError},
    model::User,
    FetchError,
};

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./gitlab_schema.json",
    query_path = "./users_query.graphql",
    response_derives = "Debug",
    "Clone",
    skip_serializing_none
)]
pub(crate) struct UsersQuery;

impl Fetch<Vec<User>, HashSet<u32>> for Fetcher {
    fn fetch(
        &self,
        client: &reqwest::blocking::Client,
        ids: HashSet<u32>,
    ) -> anyhow::Result<Vec<User>> {
        use users_query as q;

        let user_ids = ids
            .iter()
            .map(|id| format!("gid://gitlab/User/{}", id))
            .collect();

        let query = UsersQuery::build_query(q::Variables { user_ids });
        let resp = client.post(&self.base_uri).json(&query).send()?;
        let response_body: Response<q::ResponseData> = resp.json()?;
        let data: q::ResponseData = response_body
            .data
            .ok_or_else(|| FetchError::from("No data on user query response"))?;
        let users = data.users.ok_or_else(|| {
            FetchError::from("No users data on user query response's data object")
        })?;
        let nodes = users.nodes.ok_or_else(|| {
            FetchError::from("No nodes data on user query response's user object")
        })?;

        let result = nodes
            .into_iter()
            .flatten()
            .try_fold(vec![], |mut vec, node| {
                let Id(id) = Id::try_from(node.id.as_str())?;
                let user = User {
                    id,
                    username: node.username,
                };
                vec.push(user);
                Ok::<Vec<User>, IdError>(vec)
            })?;

        Ok(result)
    }
}
