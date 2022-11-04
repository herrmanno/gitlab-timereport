.mode column
.headers yes

select
    u.username as "User",
    ms.name as "Milestone",
    round(t.time / 60.0, 2) "Time (h)",
    i.iid as "Issue",
    mr.iid as "Mergerequest",
    t.date as "Date"
from TimeLog t
cross join Milestone ms
inner join User u on t.user_id = u.id
left join Issue i on t.issue_id = i.id and i.milestone_id = ms.id
left join MergeRequest mr on t.merge_request_id = mr.id and mr.milestone_id = ms.id
where i.iid is not null or mr.iid is not null
union
select
    u.username as "User",
    "No milestone" as "Milestone",
    round(t.time / 60.0, 2) "Time (h)",
    i.iid as "Issue",
    mr.iid as "Mergerequest",
    t.date as "Date"
from TimeLog t
inner join User u on t.user_id = u.id
left join Issue i on t.issue_id = i.id and i.milestone_id is null
left join MergeRequest mr on t.merge_request_id = mr.id and mr.milestone_id is null
where i.iid is not null or mr.iid is not null
order by User
;
