-- Your SQL goes here
alter table my_task
drop column if exists created_at,
drop column if exists task_at;