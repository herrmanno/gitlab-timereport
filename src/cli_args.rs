use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[arg(
        short,
        long,
        help = "GraphQL API URI. Usually something like 'https://gitlab.com/api/graphql'."
    )]
    pub uri: String,

    #[arg(
        short,
        long,
        help = "'Personal Access Token' used for fetching. See https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html."
    )]
    pub token: String,

    #[arg(short, long, help = "The name of the GitLab group to fetch")]
    pub group: String,

    #[arg(short, long, default_value_t = false, help = "Overwrite out file")]
    pub force: bool,

    #[arg()]
    pub out_file: Option<String>,
}
