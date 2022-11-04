# GitLab Time Report Crawler

A utility tool to crawl time log and other data from a GitLab instance.  
Stores data as local SQLite database, so one can run arbitrary reports on that data.

![icon](./icon.png)

## Install
```shell
cargo build [--release]
```

## Run
```shell
./target/{debug,release}/gitlab-timereport --uri <GitLab API URI> --token <GitLab personal access token> --group <GitLab group name> [db_file_name]
```
This will crawl the GitLab instance at `uri`, fetches all necessary data of the `group` and stores
it locally inside an SQLite database.

## Reports
This tool is not meant to be a full 'report generator'. Nevertheless, some example reports are
located at [reports](./reports/).

### Run Report
```shell
sqlite3 <database_file> < reports/my_report.sql
```

## Gotchas

### I don't know my GitLab's API URI

The URI of GitLab's GraphQL API usually matches `https://my_gitlab_instance.com/api/graphql`.

See [https://docs.gitlab.com/ee/api/graphql/](https://docs.gitlab.com/ee/api/graphql/) for further
information.

### Where do I get a 'personal access token' from

A personal access token is needed to fetch data from GitLab's API.  
An access token can be generated at your GitLab user's setting page.

See [https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html)
for further information.

### Error: `Error: Error while fetching API: Group response is empty`

This most likely means your group name isn't correct.
To find out the correct group name, navigate your group's page at GitLab and extract it from the
current URL.
The schema is `https://your_gitlab_instace.com/{GROUP_NAME}`.