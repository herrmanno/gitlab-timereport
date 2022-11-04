.mode column
.headers yes

select 
    User,
    Milestone,
    time_issue + time_mr as "Time (h)",
    time_issue as "Time issue (h)",
    time_mr as "Time merge request (h)"
from (
    select distinct
        u.username as user,
        ms.name as milestone,
        round(total(t.time) / 60.0, 2) as time_issue,
        0 as time_mr
    from Milestone ms
    cross join User u
    left join Issue i on i.milestone_id = ms.id
    left join TimeLog t on t.issue_id= i.id and t.user_id = u.id
    where t.time is not null
    group by Milestone, User
    union

    select distinct
        u.username as user,
        "No milestone" as milestone,
        round(total(t.time) / 60.0, 2) as time_issue,
        0 as time_mr
    from User u
    left join Issue i on i.milestone_id is null
    left join TimeLog t on t.issue_id= i.id and t.user_id = u.id
    where t.time is not null
    group by Milestone, User

    union

    select distinct
        u.username as user,
        ms.name as milestone,
        0 as time_issue,
        round(total(t.time) / 60.0, 2) as time_mr
    from Milestone ms
    cross join User u
    left join MergeRequest m on m.milestone_id = ms.id
    left join TimeLog t on t.merge_request_id= m.id and t.user_id = u.id
    where t.time is not null
    group by Milestone, User

    union

    select distinct
        u.username as user,
        "No milestone" as milestone,
        0 as time_issue,
        round(total(t.time) / 60.0, 2) as time_mr
    from User u
    left join MergeRequest m on m.milestone_id is null
    left join TimeLog t on t.merge_request_id= m.id and t.user_id = u.id
    where t.time is not null
    group by Milestone, User
)
order by "Time (h)" desc;