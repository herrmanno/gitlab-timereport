use crate::model::*;
use anyhow::Context;
use rusqlite as sql;

pub(crate) fn save_to_db(
    file_path: impl AsRef<str>,
    projects: Vec<Project>,
    milestones: Vec<Milestone>,
    issues: Vec<Issue>,
    merge_requests: Vec<MergeRequest>,
    time_logs: Vec<TimeLog>,
    users: Vec<User>,
) -> anyhow::Result<()> {
    let conn = sql::Connection::open(file_path.as_ref())?;
    create_tables(&conn)?;
    insert_users(&conn, &users)?;
    insert_projects(&conn, &projects)?;
    insert_milestones(&conn, &milestones)?;
    insert_issues(&conn, &issues)?;
    insert_merge_requests(&conn, &merge_requests)?;
    insert_time_logs(&conn, &time_logs)?;
    conn.close().map_err(|(_, e)| e)?;

    Ok(())
}

fn create_tables(conn: &sql::Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE User (id INTEGER NOT NULL PRIMARY KEY, username VARCHAR)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE Project (id INTEGER NOT NULL PRIMARY KEY, name VARCHAR)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE Milestone (id INTEGER NOT NULL PRIMARY KEY, name VARCHAR)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE Issue (
            id INTEGER NOT NULL PRIMARY KEY,
            iid INTEGER NOT NULL,
            project_id INTEGER NOT NULL,
            milestone_id INTEGER,
            name VARCHAR,
            CONSTRAINT fk_project_id FOREIGN KEY (project_id) REFERENCES Project (id),
            CONSTRAINT fk_milestone_id FOREIGN KEY (milestone_id) REFERENCES Milestone (id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE MergeRequest (
            id INTEGER NOT NULL PRIMARY KEY,
            iid INTEGER NOT NULL,
            project_id INTEGER NOT NULL,
            milestone_id INTEGER,
            name VARCHAR,
            CONSTRAINT fk_project_id FOREIGN KEY (project_id) REFERENCES Project (id),
            CONSTRAINT fk_milestone_id FOREIGN KEY (milestone_id) REFERENCES Milestone (id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE TimeLog (
            time INTEGER NOT NULL,
            date VARCHAR NOT NULL,
            user_id INTEGER NOT NULL,
            issue_id INTEGER,
            merge_request_id INTEGER,
            PRIMARY KEY (user_id, date),
            CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES User (id),
            CONSTRAINT fk_issue_id FOREIGN KEY (issue_id) REFERENCES Issue (id),
            CONSTRAINT fk_merge_request_id FOREIGN KEY (merge_request_id) REFERENCES MergeRequest (id)
        )",
        []
    )?;
    Ok(())
}

fn insert_users(conn: &sql::Connection, users: &Vec<User>) -> anyhow::Result<()> {
    for user in users {
        conn.execute(
            "INSERT INTO User VALUES (?,?)",
            sql::params![user.id, user.username],
        )
        .with_context(|| format!("Insert user {:?}", user))?;
    }
    Ok(())
}

fn insert_projects(conn: &sql::Connection, projects: &Vec<Project>) -> anyhow::Result<()> {
    for project in projects {
        conn.execute(
            "INSERT INTO Project VALUES (?,?)",
            sql::params![project.id, project.name],
        )
        .with_context(|| format!("Insert project {:?}", project))?;
    }
    Ok(())
}

fn insert_milestones(conn: &sql::Connection, milestones: &Vec<Milestone>) -> anyhow::Result<()> {
    for milestone in milestones {
        conn.execute(
            "INSERT INTO Milestone VALUES (?,?)",
            sql::params![milestone.id, milestone.name],
        )
        .with_context(|| format!("Insert milestone {:?}", milestone))?;
    }
    Ok(())
}

fn insert_issues(conn: &sql::Connection, issues: &Vec<Issue>) -> anyhow::Result<()> {
    for issue in issues {
        conn.execute(
            "INSERT INTO Issue VALUES (?,?,?,?,?)",
            sql::params![
                issue.id,
                issue.iid,
                issue.project_id,
                issue.milestone_id,
                issue.name
            ],
        )
        .with_context(|| format!("Insert issue {:?}", issue))?;
    }
    Ok(())
}

fn insert_merge_requests(
    conn: &sql::Connection,
    merge_requests: &Vec<MergeRequest>,
) -> anyhow::Result<()> {
    for merge_request in merge_requests {
        conn.execute(
            "INSERT INTO MergeRequest VALUES (?,?,?,?,?)",
            sql::params![
                merge_request.id,
                merge_request.iid,
                merge_request.project_id,
                merge_request.milestone_id,
                merge_request.name
            ],
        )
        .with_context(|| format!("Insert merge request {:?}", merge_request))?;
    }
    Ok(())
}

fn insert_time_logs(conn: &sql::Connection, time_logs: &Vec<TimeLog>) -> anyhow::Result<()> {
    for time_log in time_logs {
        conn.execute(
            "INSERT INTO TimeLog VALUES (?,?,?,?,?)",
            sql::params![
                time_log.time,
                time_log.date,
                time_log.user_id,
                time_log.issue_id,
                time_log.merge_request_id,
            ],
        )
        .with_context(|| format!("Insert time_log {:?}", time_log))?;
    }
    Ok(())
}
