-- Your SQL goes here
create table my_task(
    id serial primary key,
    task_name varchar not null,
    task_description varchar not null,
    task_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);